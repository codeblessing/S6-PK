pub use __md5_impls::*;
pub use __sha1_impls::*;
pub use __sha2_impls::*;
pub use __sha3_impls::*;

mod __sha1_impls {
    use crate::traits::HashGenerator;

    use sha1::{Digest, Sha1};

    impl HashGenerator for Sha1 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);

            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);

            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA1"
        }
    }
}

mod __sha2_impls {
    use crate::traits::HashGenerator;

    use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

    impl HashGenerator for Sha224 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA2-224"
        }
    }

    impl HashGenerator for Sha256 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }

        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA2-256"
        }
    }

    impl HashGenerator for Sha384 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA2-384"
        }
    }

    impl HashGenerator for Sha512 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA2-512"
        }
    }

    impl HashGenerator for Sha512_224 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA2-512-224"
        }
    }

    impl HashGenerator for Sha512_256 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }

        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA2-512-256"
        }
    }
}

mod __sha3_impls {
    use crate::traits::HashGenerator;

    use sha3::{
        Digest, Keccak224, Keccak256, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512,
    };

    impl HashGenerator for Keccak224 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "KECCAK-224"
        }
    }

    impl HashGenerator for Keccak256 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "KECCAK-256"
        }
    }

    impl HashGenerator for Keccak384 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "KECCAK-384"
        }
    }

    impl HashGenerator for Keccak512 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "KECCAK-512"
        }
    }

    impl HashGenerator for Sha3_224 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA3-224"
        }
    }

    impl HashGenerator for Sha3_256 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA3-256"
        }
    }

    impl HashGenerator for Sha3_384 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA3-384"
        }
    }

    impl HashGenerator for Sha3_512 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "SHA3-512"
        }
    }
}

mod __md5_impls {
    use crate::traits::HashGenerator;

    use md5::{Digest, Md5};

    impl HashGenerator for Md5 {
        fn generate(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:08b}", prev, next))
        }
        
        fn generate_hex(&mut self, message: &[u8]) -> String {
            self.reset();
            self.update(message);
            self.clone()
                .finalize()
                .into_iter()
                .fold(String::new(), |prev, next| format!("{}{:02x}", prev, next))
        }

        fn name(&self) -> &'static str {
            "MD-5"
        }
    }
}
