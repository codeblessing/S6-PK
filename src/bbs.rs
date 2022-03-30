use crate::primes::generate;
use bitvec::prelude::Msb0;
use bitvec::vec::BitVec;
use ibig::modular::ModuloRing;
use ibig::ops::RemEuclid;
use ibig::{ubig, UBig};
use rand::{thread_rng, Rng};

pub fn bbs(length: usize, prime_length: usize) -> BitVec<u8, Msb0> {
    let ring = ModuloRing::new(&ubig!(4));
    let p = loop {
        let num = generate(prime_length);
        if ring.from(num.clone()) == ring.from(3u8) {
            break num;
        }
    };
    let q = loop {
        let num = generate(1024);
        if ring.from(num.clone()) == ring.from(3u8) {
            break num;
        }
    };

    let n = p * q;

    let x = coprime(&n);

    let ring = ModuloRing::new(&n);

    let mut current = ring.from(x).pow(&ubig!(2));

    let mut output = BitVec::with_capacity(length);

    for _ in 0..length {
        let next = current.pow(&ubig!(2));
        let lsb = current.residue().to_be_bytes().pop().unwrap_or(0) & 0x01;
        output.push(lsb != 0);
        current = next;
    }

    output
}

fn coprime(max: &UBig) -> UBig {
    loop {
        let x = thread_rng().gen_range(ubig!(0)..max.clone());
        if gcd(x.clone(), max.clone()) == ubig!(1) {
            break x;
        }
    }
}

fn gcd(mut a: UBig, mut b: UBig) -> UBig {
    while b != ubig!(0) {
        let c = a.rem_euclid(&b);
        a = b;
        b = c;
    }

    a
}
