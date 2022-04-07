use ibig::modular::ModuloRing;
use ibig::ops::RemEuclid;
use ibig::{ibig, ubig, IBig, UBig};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::thread_rng;

use crate::primes;

struct Key<T> {
    exponent: T,
    module: T,
}

#[non_exhaustive]
pub struct PublicKey(Key<UBig>);

impl PublicKey {
    fn new(exponent: UBig, module: UBig) -> Self {
        Self(Key { exponent, module })
    }

    pub fn encrypt(&self, message: &[u8]) -> Vec<u8> {
        let ring = ModuloRing::new(&self.0.module);
        let block_size = (self.0.module.bit_len() as f64 / 8.0 - 0.1).floor() as usize;

        let mut encrypted = Vec::with_capacity(message.len());

        for block in message.chunks(block_size) {
            let chunk = UBig::from_le_bytes(block);

            let encrypted_block = ring.from(chunk).pow(&self.0.exponent).residue();

            let mut encrypted_block = encrypted_block.to_le_bytes();

            encrypted.append(&mut encrypted_block);
        }

        encrypted
    }
}

#[non_exhaustive]
pub struct PrivateKey(Key<UBig>);

impl PrivateKey {
    fn new(exponent: UBig, module: UBig) -> Self {
        Self(Key { exponent, module })
    }

    pub fn decrypt(&self, message: &[u8]) -> Vec<u8> {
        let ring = ModuloRing::new(&self.0.module);
        let block_size = (self.0.module.bit_len() as f64 / 8.0).ceil() as usize;

        let mut decrypted = Vec::with_capacity(message.len());

        for block in message.chunks(block_size) {
            let chunk = UBig::from_le_bytes(block);

            let decrypted_block = ring.from(chunk).pow(&self.0.exponent).residue();

            let mut decrypted_block = decrypted_block.to_le_bytes();

            decrypted.append(&mut decrypted_block);
        }

        decrypted
    }
}

pub fn generate_key_pair(length: usize) -> (PublicKey, PrivateKey) {
    let p = primes::generate(length);
    let q = primes::generate(length);

    let n = p.clone() * q.clone();
    let phi = (p - 1u8) * (q - 1u8);

    let e = generate_coprime_to(&phi);
    let d = find_inverse_in_modulo_ring(e.clone(), phi.clone());

    return (PublicKey::new(e, n.clone()), PrivateKey::new(d, n));
}

fn generate_coprime_to(target: &UBig) -> UBig {
    fn _gcd(mut a: UBig, mut b: UBig) -> UBig {
        while b != ubig!(0) {
            let c = a.rem_euclid(&b);
            (a, b) = (b, c);
        }

        a
    }

    let distribution = Uniform::new(ubig!(0), target.clone());
    let mut generator = thread_rng();

    loop {
        let candidate = distribution.sample(&mut generator);
        if _gcd(candidate.clone(), target.clone()) == ubig!(1) {
            break candidate;
        }
    }
}

fn find_inverse_in_modulo_ring(mut target: UBig, mut module: UBig) -> UBig {
    let ring = ModuloRing::new(&module);

    let mut q: IBig = (&module / &target).into();
    let mut r = module.rem_euclid(&target);
    let mut t;
    let mut x = ibig!(1);
    let mut z = -q;

    while &r != &ubig!(0) {
        module = target;
        target = r;
        q = (&module / &target).into();
        r = module.rem_euclid(&target);
        t = x;
        x = z;
        z = &t - (q * &x);
    }

    ring.from(x).residue()
}

#[cfg(test)]
mod test_rsa {
    use super::*;

    #[test]
    fn test_inverse() {
        let result = find_inverse_in_modulo_ring(ubig!(11), ubig!(26));

        assert_eq!(result, ubig!(19));
    }
}
