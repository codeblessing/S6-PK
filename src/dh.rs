//! Simple Diffie-Hellman key exchange algorithm implementation.
//!
//! Because DH key exchange safety does not depend on chosen
//! prime and generator numbers (as they are public) this implementation
//! uses numbers defined in [RFC 2409 (Internet Key Exchange) §6.2](https://www.rfc-editor.org/rfc/rfc2409#section-6.2)
// Based on https://crypto.stackexchange.com/questions/820/how-does-one-calculate-a-primitive-root-for-diffie-hellman

use ibig::modular::{Modulo, ModuloRing};
use ibig::{ubig, UBig};
use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref N: UBig = ubig!(_FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE65381FFFFFFFFFFFFFFFF base 16);
    static ref RING: ModuloRing = ModuloRing::new(&N);
    static ref G: Modulo<'static> = RING.from(ubig!(2));
}

pub struct Key {
    private: UBig,
    public: UBig,
    session: Option<UBig>,
}

impl Key {
    pub fn create_exchange_keys(key_size: usize) -> Self {
        let lower = {
            let mut l = ubig!(1);
            l.set_bit(key_size);
            l
        };
        let upper = {
            let mut u = ubig!(1);
            u.set_bit(key_size + 1);
            u
        };
        let private = rand::thread_rng().gen_range(lower..upper);
        let public = G.pow(&private).residue();

        Self {
            private,
            public,
            session: None,
        }
    }

    pub fn create_session_key(&mut self, public_key: &UBig) {
        let session = RING.from(public_key).pow(&self.private).residue();
        self.session = Some(session)
    }

    pub fn get_public_key(&self) -> &UBig {
        &self.public
    }

    pub fn get_session_key(&self) -> Option<&UBig> {
        self.session.as_ref()
    }
}
