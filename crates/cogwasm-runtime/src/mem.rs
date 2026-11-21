//! Guest memory access shared by the hosts.

use anyhow::{Context, Result, bail};
use wasmtime::{Caller, Extern, Memory};

/// The largest NUL-terminated string a host will read out of guest memory.
///
/// A guest that never terminates its string is a bug in the guest; silently
/// truncating at the cap would turn that bug into a wrong host path or linker
/// argument, so a longer string is an error instead.
pub const MAX_CSTR_BYTES: usize = 1 << 20;

/// A checked view of a guest instance's exported linear memory.
#[derive(Clone)]
pub struct GuestMem(Memory);

impl GuestMem {
    /// Read `buf` from guest memory at `offset`, adding context to an
    /// out-of-bounds access.
    pub fn read<T>(&self, caller: &Caller<'_, T>, offset: usize, buf: &mut [u8]) -> Result<()> {
        self.0
            .read(caller, offset, buf)
            .with_context(|| format!("reading guest memory at {offset:#x}"))
    }

    /// Read a NUL-terminated UTF-8 string starting at `pointer`.
    ///
    /// Bytes past the terminator are ignored. A string without a terminator
    /// within [`MAX_CSTR_BYTES`] bytes is an error, as is a pointer outside
    /// guest memory.
    pub fn read_cstr<T>(&self, caller: &Caller<'_, T>, pointer: u32) -> Result<String> {
        let mut bytes = Vec::new();
        let mut offset = usize::try_from(pointer).context("guest string pointer")?;
        loop {
            let mut byte = [0u8; 1];
            self.read(caller, offset, &mut byte)
                .with_context(|| format!("reading guest string at {pointer:#x}"))?;
            if byte[0] == 0 {
                break;
            }
            bytes.push(byte[0]);
            offset += 1;
            if bytes.len() > MAX_CSTR_BYTES {
                bail!("guest string at {pointer:#x} is longer than 1 MiB");
            }
        }
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }
}

/// Find the guest instance's exported `memory`.
pub fn guest_memory<T>(caller: &mut Caller<'_, T>) -> Result<GuestMem> {
    match caller.get_export("memory") {
        Some(Extern::Memory(memory)) => Ok(GuestMem(memory)),
        Some(_) => bail!("guest module export `memory` is not a memory"),
        None => bail!("guest module does not export memory"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Wt;
    use anyhow::{Result, anyhow};
    use wasmtime::{Engine, Linker, Module, Store};

    /// A module with one imported host function `env.probe`, a 17-page memory
    /// exported as `memory`, and a wasm `run` function that calls `probe`.
    /// Calling `run` enters `probe` with a real `Caller` for this instance.
    const MEMORY_MODULE: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // magic and version
        0x01, 0x04, 0x01, 0x60, 0x00, 0x00, // type: () -> ()
        0x02, 0x0d, 0x01, 0x03, b'e', b'n', b'v', 0x05, b'p', b'r', b'o', b'b', b'e', 0x00,
        0x00, // import env.probe
        0x03, 0x02, 0x01, 0x00, // function: one function of type 0
        0x05, 0x03, 0x01, 0x00, 0x11, // memory: min 17 pages
        0x07, 0x10, 0x02, 0x06, b'm', b'e', b'm', b'o', b'r', b'y', 0x02, 0x00, 0x03, b'r', b'u',
        b'n', 0x00, 0x01, // exports memory and run
        0x0a, 0x06, 0x01, 0x04, 0x00, 0x10, 0x00, 0x0b, // run: call 0; end
    ];

    /// Instantiate [`MEMORY_MODULE`], fill guest memory at `offset`, then call
    /// `probe` from inside the instance.
    fn with_guest_memory(
        contents: &[u8],
        offset: usize,
        probe: impl Fn(Caller<'_, ()>) + Send + Sync + 'static,
    ) -> Result<()> {
        let engine = Engine::default();
        let module = Module::new(&engine, MEMORY_MODULE).wt("compiling test module")?;
        let mut store = Store::new(&engine, ());
        let mut linker = Linker::new(&engine);
        linker
            .func_wrap("env", "probe", probe)
            .wt("defining test probe")?;
        let instance = linker
            .instantiate(&mut store, &module)
            .wt("instantiating test module")?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| anyhow!("test module does not export memory"))?;
        memory.write(&mut store, offset, contents)?;
        instance
            .get_typed_func::<(), ()>(&mut store, "run")
            .wt("test module run export")?
            .call(&mut store, ())
            .wt("calling test probe")?;
        Ok(())
    }

    #[test]
    fn reads_a_nul_terminated_string() -> Result<()> {
        with_guest_memory(b"hello\0ignored", 0x100, |mut caller| {
            let memory = guest_memory(&mut caller).expect("guest memory");
            assert_eq!(
                memory.read_cstr(&caller, 0x100).expect("read string"),
                "hello"
            );
        })
    }

    #[test]
    fn rejects_a_string_longer_than_the_cap() -> Result<()> {
        let unterminated = vec![b'a'; MAX_CSTR_BYTES + 1];
        with_guest_memory(&unterminated, 0, |mut caller| {
            let memory = guest_memory(&mut caller).expect("guest memory");
            let error = memory
                .read_cstr(&caller, 0)
                .expect_err("an unterminated string must be an error");
            assert!(
                error.to_string().contains("longer than 1 MiB"),
                "unexpected error: {error:#}"
            );
        })
    }

    #[test]
    fn rejects_a_pointer_outside_guest_memory() -> Result<()> {
        // The module exports exactly 17 pages, so this is the first byte past
        // the end of memory.
        const PAST_THE_END: u32 = 17 * 65_536;
        with_guest_memory(b"unused\0", 0, |mut caller| {
            let memory = guest_memory(&mut caller).expect("guest memory");
            assert!(
                memory.read_cstr(&caller, PAST_THE_END).is_err(),
                "a pointer past the end of memory must be an error"
            );
        })
    }
}
