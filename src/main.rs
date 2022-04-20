#![deny(clippy::pedantic)]

use crate::dh::Key;
mod primes;
mod rsa;
mod dh;



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

    // RSA
    let (public_key, private_key) = rsa::generate_key_pair(length);
    let mut message = String::with_capacity(100);
    std::io::stdin().read_line(&mut message).expect("Cannot read message from stdin!");

    let encrypted = public_key.encrypt(message.as_bytes());
    let encrypted_string = String::from_utf8_lossy(&encrypted);

    println!("Encrypted: {encrypted_string}");

    let decrypted = private_key.decrypt(&encrypted);
    let msg = String::from_utf8_lossy(&decrypted);

    println!("Decrypted: {msg}");

    assert_eq!(&message, &msg);

    // Diffie-Hellman key exchange:

    let mut alice = Key::create_exchange_keys(length);
    let mut bob = Key::create_exchange_keys(length);
    alice.create_session_key(bob.get_public_key());
    bob.create_session_key(alice.get_public_key());

    let alice_session = alice.get_session_key().expect("Alice session key exploded!");
    let bob_session = bob.get_session_key().expect("Bob session key exploded!");

    println!("Alice's session key: {alice_session}");
    println!("Bob's session key:   {bob_session}");

    assert_eq!(alice_session, bob_session);
}

