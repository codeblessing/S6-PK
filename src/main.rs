mod error;
mod playfair;

use std::io::{BufRead, Write};

use playfair::Playfair;

enum State {
    Encrypt,
    Decrypt,
    ChangeKey,
}

fn main() {
    const MESSAGE: &str = r"
Playfair v1.0.0-20220305

Welcome to Playfair Crypto-coder.
This tool allows you to encrypt and decrypt messages using playfair method.

USAGE:
Upon start this program will ask you to specify keyword used to encrypt messages.
Next you need to choose action:
    e - encrypt message
    d - decrypt message
    k - change keyword
    q - quit
Finally you need to insert your message with # on end.

Have a great work!
";

    println!("{}", MESSAGE);
    let mut state;
    let mut coder: Playfair = Playfair::new("").unwrap();
    set_keyword(&mut coder);

    loop {
        print!("Select action [(e)ncrypt, (d)ecrypt, (q)uit, change (k)eyword: ");
        #[allow(unused_must_use)]
        {
            std::io::stdout().flush();
        }
        let mut buffer = String::new();
        match std::io::stdin().lock().read_line(&mut buffer) {
            Ok(_) => match buffer.trim() {
                "e" => state = State::Encrypt,
                "d" => state = State::Decrypt,
                "k" => state = State::ChangeKey,
                "q" => return,
                _ => {
                    eprintln!("Unknown action. Try again.");
                    continue;
                }
            },
            Err(_) => {
                eprintln!("Unexpected error while reading from stdin.");
                continue;
            }
        }

        match state {
            State::ChangeKey => set_keyword(&mut coder),
            State::Encrypt => {
                let message = read_message();
                let encrypted = coder.encrypt(&message);
                println!("Encrypted message:\n");
                println!("{}\n", encrypted);
            }
            State::Decrypt => {
                let message = read_message();
                let decrypted = coder.decrypt(&message);
                println!("Decrypted message:\n");
                println!("{}\n", decrypted);
            }
        }
    }
}

fn read_message() -> String {
    loop {
        println!("Message:");
        #[allow(unused_must_use)]
        {
            std::io::stdout().flush();
        }
        let mut message = String::new();
        let input = std::io::stdin();
        for line in input.lock().lines() {
            match line {
                Ok(line) => {
                    if line.contains('#') {
                        let line = line.split('#').next().unwrap();
                        message.push_str(line);
                        break;
                    } else {
                        message.push_str(line.as_str());
                    }
                }
                Err(_) => continue,
            }
        }

        return message;
    }
}

fn set_keyword(coder: &mut Playfair) {
    loop {
        print!("Keyword: ");
        #[allow(unused_must_use)]
        {
            std::io::stdout().flush();
        }
        let mut keyword = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut keyword)
            .expect("Unexpected error. Reading from stdin failed.");
        keyword.pop();
        match Playfair::new(&keyword) {
            Ok(mut new_coder) => std::mem::swap(coder, &mut new_coder),
            Err(_) => {
                eprintln!("Keyword contains unallowed characters. Only allowed characters are ASCII letters.\nPlease try again.");
                continue;
            }
        };
        break;
    }
}

// 300 wyrazów -> szyfrogram
