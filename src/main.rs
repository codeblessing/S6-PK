mod primes;
mod modulo;
mod shares;

use clap::{ArgEnum, Parser};

fn main() {}

fn lagrange_polynomial(index: usize, xs: &[i128], prime: u128) -> i128 {
    fn find_inverse_in_modulo_ring(mut target: u128, mut module: u128) -> u128 {
        let ring = module;

        let mut q = module / target;
        let mut r = module.rem_euclid(target);
        let mut t;
        let mut x = 1;
        let mut z = -(q as i128);

        while r != 0 {
            module = target;
            target = r;
            q = module / target;
            r = module.rem_euclid(target);
            t = x;
            x = z;
            z = t - (q as i128 * x);
        }

        x.rem_euclid(ring as i128) as u128
    }

    let nominator: u128 = xs
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, value)| -value)
        .product::<i128>()
        .rem_euclid(prime as i128) as u128;

    let x_i = xs[index - 1];
    let denominator: u128 = xs
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, value)| x_i - value)
        .product::<i128>()
        .rem_euclid(prime as i128) as u128;

    let denominator = find_inverse_in_modulo_ring(denominator, prime);

    (nominator * denominator).rem_euclid(prime) as i128
}

fn recreate_secret(xs: &[i128], shares: &[i128], prime: u128) -> i128 {
    xs.iter()
        .zip(shares)
        .map(|(i, s)| {
            let polynomial = lagrange_polynomial(*i as usize, xs, prime);
            polynomial * s
        })
        .sum::<i128>()
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

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_recreate_secret() {
        let xs = [1, 2, 3];
        let shares = [1368, 383, 1045];
        let prime = 1523;

        let secret = recreate_secret(&xs, &shares, prime);

        assert_eq!(secret, 954i128);
    }
}