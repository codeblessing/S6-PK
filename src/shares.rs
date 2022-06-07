use rand::prelude::random;
use crate::primes;

pub fn make_shares(secret: u64, share_count: usize, recreate_share_count: usize) -> Vec<(usize, i64)> {
    let coefficients = (1..recreate_share_count).map(|_| random::<u32>()).collect::<Vec<_>>();
    let prime = primes::generate(64);
    todo!();
}