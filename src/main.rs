#![deny(clippy::pedantic)]
mod primes;
mod rsa;



fn main() {
    let length = {
        let mut buf = String::with_capacity(10);
        loop {
            buf.clear();
            std::io::stdin()
                .read_line(&mut buf)
                .expect("cannot read from stdin");
            let sanitized = buf.trim();
            let len = sanitized.parse::<usize>().unwrap_or(0);
            if len != 0 {
                break len;
            } else {
                println!("Invalid number. Please insert correct number.")
            }
        }
    };

    let (public_key, private_key) = rsa::generate_key_pair(length);

    let message = "Hello, my dear students";

    let encrypted = public_key.encrypt(message.as_bytes());

    // let message: [u8; 5] = [1, 2, 3, 4, 5];
    // let encrypted = public_key.encrypt(&message);

    println!("Encrypted: {encrypted:?}");

    let decrypted = private_key.decrypt(&encrypted);

    let msg = String::from_utf8_lossy(&decrypted);

    println!("Decrypted: {msg}");
}

