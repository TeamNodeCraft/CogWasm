// E1 probe: how long does rustc's wasm32 `Mmap::map` (File + read_to_end into a
// Vec) take for each sysroot rmeta, inside the same engine/WASI setup as rustc.wasm?
use std::io::Read;
use std::time::Instant;

fn pages() -> usize { core::arch::wasm32::memory_size(0) }

fn read_like_rustc(path: &str) -> (Vec<u8>, f64) {
    let t = Instant::now();
    let mut f = std::fs::File::open(path).unwrap();
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();
    (data, t.elapsed().as_secs_f64() * 1e3)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rounds: usize = args[0].parse().unwrap();
    let files = &args[1..];
    let p0 = pages();
    for round in 0..rounds {
        let mut total = 0.0;
        let mut kept = Vec::new();
        for f in files {
            let (data, ms) = read_like_rustc(f);
            // Touch one byte per page so the read cannot be skipped.
            let mut sum = 0u64;
            for i in (0..data.len()).step_by(4096) { sum = sum.wrapping_add(data[i] as u64); }
            let name = f.rsplit('/').next().unwrap();
            println!("round {round} {name:48} {:>10} B  {ms:7.2} ms  (sum {sum})", data.len());
            total += ms;
            kept.push(data); // rustc keeps every blob alive for the session
        }
        println!("round {round} total {total:.2} ms, memory {} -> {} pages ({} MiB)",
                 p0, pages(), pages() * 64 / 1024);
        drop(kept);
    }
}
