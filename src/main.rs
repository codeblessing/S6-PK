mod collisions;
mod hashes;
mod perf;
mod sac;
mod traits;

use std::io::Write;
use std::path::Path;

use clap::{ArgEnum, Parser};

use md5::Md5;
use sha1::{Digest, Sha1};
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Keccak224, Keccak256, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use traits::HashGenerator;

fn main() {
    let args = Args::parse();

    let mut algs: Vec<Box<dyn HashGenerator>> = match args.algoritm {
        Algorithm::ALL => vec![
            Box::new(Sha1::new()),
            Box::new(Sha224::new()),
            Box::new(Sha256::new()),
            Box::new(Sha384::new()),
            Box::new(Sha512::new()),
            Box::new(Sha512_224::new()),
            Box::new(Sha512_256::new()),
            Box::new(Keccak224::new()),
            Box::new(Keccak256::new()),
            Box::new(Keccak384::new()),
            Box::new(Keccak512::new()),
            Box::new(Sha3_224::new()),
            Box::new(Sha3_256::new()),
            Box::new(Sha3_384::new()),
            Box::new(Sha3_512::new()),
            Box::new(Md5::new()),
        ],
        Algorithm::SHA1 => vec![Box::new(Sha1::new())],
        Algorithm::SHA2 => vec![
            Box::new(Sha224::new()),
            Box::new(Sha256::new()),
            Box::new(Sha384::new()),
            Box::new(Sha512::new()),
            Box::new(Sha512_224::new()),
            Box::new(Sha512_256::new()),
        ],
        Algorithm::SHA3 => vec![
            Box::new(Keccak224::new()),
            Box::new(Keccak256::new()),
            Box::new(Keccak384::new()),
            Box::new(Keccak512::new()),
            Box::new(Sha3_224::new()),
            Box::new(Sha3_256::new()),
            Box::new(Sha3_384::new()),
            Box::new(Sha3_512::new()),
        ],
        Algorithm::MD5 => vec![Box::new(Md5::new())],
    };

    let data = read_file(args.input_file);
    let data = if args.by_line {
        data.lines().map(ToOwned::to_owned).collect()
    } else {
        vec![data]
    };

    if args.tests {
        for generator in algs.iter_mut() {
            let collisions = collisions::find_collision_on(12, generator.as_mut(), data.as_ref());
            let times = perf::performance(generator.as_mut());
            let sac = sac::check_sac(generator.as_mut(), data.as_ref());
            let path = format!("output/{}.txt", generator.name());
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(false)
                .create(true)
                .open(path)
                .expect("Could not create output file.");
            {
                let data = format!("{}\n", collisions.values().sum::<usize>());
                file.write_all(data.as_bytes())
                    .expect("Could not write into output file");
            }
            for (_, time) in ["Tiny", "Small", "Mid", "Big", "Huge", "Gigantic"]
                .iter()
                .zip(times.iter())
            {
                let data = format!("{}\n", time);
                file.write_all(data.as_bytes())
                    .expect("Could not write into output file");
            }
            {
                let data = format!("{}", sac);
                file.write_all(data.as_bytes())
                    .expect("Could not write into output file");
            }
            let path = format!("output/{}-details.txt", generator.name());
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(false)
                .create(true)
                .open(path)
                .expect("Could not create output file.");
            for key in 0..4096 {
                let key = format!("{:012b}", key);
                file.write_all(
                    format!("{}\n", collisions.get(&key).or(Some(&0)).unwrap()).as_bytes(),
                )
                .expect("Could not write into output file");
            }
        }
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open("output/-details.txt")
            .expect("Could not create output file.");
        for key in 0..4096 {
            let key = format!("{:012b}", key);
            file.write_all(format!("\"{}\"\n", key).as_bytes())
                .expect("Could not write into output file");
        }
    } else {
        for message in &data {
            for generator in algs.iter_mut() {
                let name = generator.name();
                let hash = generator.generate_hex(message.as_bytes());
                println!("{name:12}: {hash}");
            }
        }
    }
}

fn read_file(filepath: impl AsRef<Path>) -> String {
    std::fs::read_to_string(filepath).expect("Cannot read file into string")
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(
        short = 'l',
        long = "line-by-line",
        help = "Treat every line from file as separate input."
    )]
    by_line: bool,

    #[clap(arg_enum, short = 'a', long, default_value = "all")]
    algoritm: Algorithm,

    #[clap(help = "Data source file. Have to be UTF-8 text file.")]
    input_file: String,

    #[clap(short = 't', long)]
    tests: bool,
}

#[derive(ArgEnum, Clone, Copy)]
enum Algorithm {
    ALL,
    SHA1,
    SHA2,
    SHA3,
    MD5,
}
