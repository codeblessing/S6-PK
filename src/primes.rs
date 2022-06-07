use ibig::modular::{IntoModulo, ModuloRing};
use ibig::{ubig, UBig};
use mod_exp::mod_exp;
use rand::{thread_rng, Rng};

pub fn generate(min: u64) -> u128 {
    loop {
        let mut number = thread_rng().gen_range(u128::from(min)..u128::MAX);

        if is_prime(number) {
            break number;
        }
    }
}

fn is_prime(number: u128) -> bool {
    little_fermat(number) && miller_rabin(number, 5)
}

fn little_fermat(candidate: u128) -> bool {
    let random = thread_rng().gen_range(0..candidate);
    let result = mod_exp(random, candidate - 1, candidate);
    result == 1
}

fn miller_rabin(candidate: u128, limit: usize) -> bool {
    fn __find_divider(num: u128) -> (u128, usize) {
        let mut exp = num - 1;

        while !exp.is_power_of_two() {
            exp >>= 1;
        }

        let s = 127;
        let d = (num - 1) / exp;

        (d, s)
    }

    let (d, s) = __find_divider(candidate);

    for _ in 0..limit {
        let a = thread_rng().gen_range(0..candidate);
        let a = mod_exp(a, d, candidate);

        if a == 1 {
            continue;
        }

        let composite = (0..s).all(|r| {
            #[allow(clippy::cast_possible_truncation)]
            let exponent = 2u128.pow(r as u32);
            let x = mod_exp(a, exponent, candidate);
            x != (-1i128).rem_euclid(candidate as i128) as u128
        });

        if composite {
            return false;
        }
    }

    true
}
