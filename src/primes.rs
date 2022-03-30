use ibig::modular::{IntoModulo, ModuloRing};
use ibig::{ubig, UBig};
use rand::{thread_rng, Rng};

pub fn generate(length: usize) -> UBig {
    let mut lower = ubig!(0);
    lower.set_bit(length - 1);

    let mut upper = ubig!(0);
    upper.set_bit(length);

    loop {
        let mut number = thread_rng().gen_range(lower.clone()..upper.clone());
        number.set_bit(0);
        number.set_bit(length - 1);

        if is_prime(&number) {
            break number;
        }
    }
}

fn is_prime(number: &UBig) -> bool {
    little_fermat(number) && miller_rabin(number, 5)
}

fn little_fermat(candidate: &UBig) -> bool {
    let ring = ModuloRing::new(candidate);
    let random = thread_rng()
        .gen_range(ubig!(0)..candidate.clone())
        .into_modulo(&ring);
    let result = random.pow(&(candidate - ubig!(1)));
    result == ubig!(1).into_modulo(&ring)
}

fn miller_rabin(candidate: &UBig, limit: usize) -> bool {
    fn __find_divider(num: &UBig) -> (UBig, usize) {
        let mut exp = num - 1u8;

        while !exp.is_power_of_two() {
            exp >>= 1;
        }

        let s = exp.bit_len() - 1;
        let d = (num - 1u8) / exp;

        (d, s)
    }

    let (d, s) = __find_divider(candidate);

    let ring = ModuloRing::new(candidate);
    for _ in 0..limit {
        let a = thread_rng()
            .gen_range(ubig!(0)..candidate.clone())
            .into_modulo(&ring);
        let a = a.pow(&d);

        if a == ring.from(1) {
            continue;
        }

        let composite = (0..s).all(|r| {
            #[allow(clippy::cast_possible_truncation)]
            let exponent = 2u128.pow(r as u32);
            let x = a.pow(&UBig::from(exponent));
            x != ring.from(-1)
        });

        if composite {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test_primes {
    use super::*;

    #[test]
    fn test_bit_len() {
        let x = ubig!(0b0000110011);
        let y = ubig!(0b100100);

        assert_eq!(x.bit_len(), 6);
        assert_eq!(y.bit_len(), 6);
    }
}
