use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

use crate::traits::HashGenerator;

pub fn performance(generator: &mut dyn HashGenerator) -> Vec<f64> {
    const MAX_FILE_SIZE: usize = 100 * 1024 /* 1MB */ * 1024 /* 1kB */;
    const LOOP_ITERATIONS: usize = 10;

    let mut average_times = Vec::with_capacity(5);
    let mut buffer = Vec::with_capacity(MAX_FILE_SIZE);

    for filename in [
        "perf_tiny.txt",
        "perf_small.txt",
        "perf_mid.txt",
        "perf_big.txt",
        "perf_huge.txt",
        "perf_gigantic.txt"
    ] {
        read_data_file(&mut buffer, filename);

        let mut times: Vec<u128> = Vec::with_capacity(LOOP_ITERATIONS);
        for _ in 0..LOOP_ITERATIONS {
            let start = Instant::now();
            let _x = generator.generate(&buffer);
            let end = Instant::now();
            let span = (end - start).as_nanos();
            times.push(span);
        }
        let avg: f64 = times
            .iter()
            .map(|&time| time as f64 / LOOP_ITERATIONS as f64)
            .sum();
        average_times.push(avg);
    }

    average_times
}

fn read_data_file(/* out */ buffer: &mut Vec<u8>, filename: impl AsRef<str>) {
    let filename = filename.as_ref();
    let filepath = PathBuf::from(format!("data/{filename}"));
    buffer.clear();
    std::fs::OpenOptions::new()
        .read(true)
        .open(filepath.as_path())
        .expect(&format!("cannot open {filename}"))
        .read_to_end(buffer)
        .expect(&format!("cannot read {filename} into buffer"));
}
