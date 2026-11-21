//! Run `mem_grow.wasm` and check `memory.size` / `memory.grow` semantics:
//! initial size, a successful grow and the returned old size, the size
//! afterwards, growth by zero, a failed grow, and that the new pages are
//! readable and writable from both the host and the guest.
//!
//!     mem_grow_harness path/to/mem_grow.wasm

use anyhow::{anyhow, bail, Context, Result};
use wasmtime::{Engine, Instance, Linker, Module, Store};

/// Wasmtime 50's errors do not implement `std::error::Error`, so convert them
/// explicitly instead of using `anyhow`'s `Context`.
trait Wt<T> {
    fn wt(self, what: &'static str) -> Result<T>;
}

impl<T> Wt<T> for wasmtime::Result<T> {
    fn wt(self, what: &'static str) -> Result<T> {
        self.map_err(|e| anyhow!("{what}: {e}"))
    }
}

const PAGE: usize = 65536;

fn check(what: &str, got: i64, want: i64) -> Result<()> {
    if got != want {
        bail!("{what}: got {got}, want {want}");
    }
    println!("ok  {what}: {got}");
    Ok(())
}

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .context("usage: mem_grow_harness <mem_grow.wasm>")?;
    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).wt("load wasm")?;
    let mut store = Store::new(&engine, ());
    let instance: Instance = Linker::new(&engine)
        .instantiate(&mut store, &module)
        .wt("instantiate")?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .context("memory export")?;
    let pages = instance
        .get_typed_func::<(), i32>(&mut store, "pages")
        .wt("get pages")?;
    let grow = instance
        .get_typed_func::<i32, i32>(&mut store, "grow")
        .wt("get grow")?;
    let probe = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "probe")
        .wt("get probe")?;

    let initial = pages.call(&mut store, ()).wt("call pages")? as i64;
    check("initial memory.size", initial, memory.size(&store) as i64)?;
    if initial <= 0 {
        bail!("initial memory size is not positive: {initial}");
    }

    let old = grow.call(&mut store, 2).wt("call grow")? as i64;
    check("memory.grow(2) returns the old size", old, initial)?;
    check(
        "memory.size after growing by 2",
        pages.call(&mut store, ()).wt("call pages")? as i64,
        initial + 2,
    )?;
    check(
        "host memory.size after growing by 2",
        memory.size(&store) as i64,
        initial + 2,
    )?;

    // The first new page is `old` pages in. Write its first and last bytes
    // from the host and overwrite one of them from the guest.
    let new_page = (old as usize) * PAGE;
    memory
        .write(&mut store, new_page, b"first")
        .context("host write at first byte of new page")?;
    memory
        .write(&mut store, new_page + PAGE - 8, b"last")
        .context("host write at last byte of new page")?;
    let mut buf = [0u8; 5];
    memory
        .read(&store, new_page, &mut buf)
        .context("host read at first byte of new page")?;
    if &buf != b"first" {
        bail!("host round trip in new page returned {buf:?}");
    }
    println!("ok  host read/write in new page at {new_page:#x}");

    let guest_addr = (new_page + PAGE - 16) as i32;
    check(
        "guest write/read through a pointer in the new page",
        probe
            .call(&mut store, (guest_addr, 0x1234_5678))
            .wt("call probe")? as i64,
        0x1234_5678,
    )?;
    let mut word = [0u8; 4];
    memory
        .read(&store, guest_addr as usize, &mut word)
        .context("host read of guest write")?;
    check(
        "host sees the guest write",
        i32::from_le_bytes(word) as i64,
        0x1234_5678,
    )?;

    // Growing by zero reports the current size and does not change it.
    check(
        "memory.grow(0) returns the current size",
        grow.call(&mut store, 0).wt("call grow")? as i64,
        initial + 2,
    )?;
    check(
        "memory.size after growing by 0",
        pages.call(&mut store, ()).wt("call pages")? as i64,
        initial + 2,
    )?;

    // A grow beyond the 4 GiB memory32 limit fails with -1 and changes nothing.
    check(
        "memory.grow(65536) fails with -1",
        grow.call(&mut store, 65536).wt("call grow")? as i64,
        -1,
    )?;
    check(
        "memory.size after a failed grow",
        pages.call(&mut store, ()).wt("call pages")? as i64,
        initial + 2,
    )?;
    check(
        "host memory.size after a failed grow",
        memory.size(&store) as i64,
        initial + 2,
    )?;

    println!(
        "PASS mem_grow: initial={initial} pages, grew to {} pages",
        initial + 2
    );
    Ok(())
}
