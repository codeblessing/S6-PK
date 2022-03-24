use ibig::{ubig, UBig};
use rand::{thread_rng, Rng};

pub fn generate(length: usize) -> UBig {
    let mut lower = ubig!(0);
    lower.set_bit(length - 1);

    let upper = ubig!(0);
    lower.set_bit(length);

    let mut number = thread_rng().gen_range(lower..upper);
    number.set_bit(0);
    number.set_bit(length - 1);

    number
}

fn little_fermat(candidate: UBig) -> bool {
    let mut random = thread_rng().gen_range(ubig!(0)..candidate);
    let result = Int::modpow(&random, &(candidate - &Int::one()), candidate);
    result == Int::one()
}