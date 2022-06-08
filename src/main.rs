mod modulo;
mod primes;
mod shares;

use clap::{ArgEnum, Parser};

fn main() {
    let (prime, shares) = shares::make_shares(125, 15, 4);
    println!("{prime}");
    println!("{:?}", shares);
    // let shares = shares.iter().take(4).cloned().collect::<Vec<_>>();
    
    let prime = 13397135038569096677;
    let shares = [
        (12, 4550918075273),
        (8, 1422634082533),
        (14, 7114094491783),
        (10, 2691363908595),
    ];


    let secret = shares::recreate_secret(prime, &shares);
    println!("Recreated secret: {secret}");
}

#[derive(Parser)]
struct Args {
    #[clap(arg_enum, short, long)]
    mode: Mode,

    #[clap(short, long, help = "Total number of shares")]
    total: u32,

    #[clap(short, long, help = "Number of shares required to retrieve the secret")]
    required: u32,

    #[clap(long)]
    secret: Option<i128>,

    #[clap(long)]
    number: Option<u32>,

    #[clap(short, long)]
    share: i128,
}

#[derive(ArgEnum, Clone, Copy)]
enum Mode {
    Divide,
    Retrieve,
}
