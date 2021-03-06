use deunicode::deunicode;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("playfair key can contain only ASCII-alphabetic characters")]
pub struct NonAsciiKey;

#[derive(Clone, Debug, PartialEq)]
enum Text {
    Word(String),
    Punctuation(String),
}

pub struct Playfair {
    key: Vec<char>,
}

impl Playfair {
    fn generate_key(keyword: &str) -> Result<Vec<char>, NonAsciiKey> {
        // Check whether key contains only ASCII letters.
        // If there's non-ASCII-alphabetic throw an error.
        if keyword.chars().any(|letter| !letter.is_ascii_alphabetic()) {
            return Err(NonAsciiKey);
        }

        let mut final_key: Vec<char> = Vec::with_capacity(25);

        // Iterate over letters from keyword and alphabet and add missing letters to key.
        keyword
            .chars()
            .map(char::to_lowercase)
            .flatten()
            // replace all occurences of 'j' with 'i'
            .map(|letter| if letter == 'j' { 'i' } else { letter })
            .chain([
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
                'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ])
            .for_each(|letter| {
                if !final_key.contains(&letter) {
                    final_key.push(letter)
                }
            });

        Ok(final_key)
    }

    fn sanitize_message(message: &str) -> Vec<Text> {
        fn process_word(word: Text) -> Text {
            match word {
                Text::Word(text) => {
                    // We need to do two things:
                    // 1) Separate pairs of same letters with 'x' (but only if they are in the same pair chunk)
                    // [let `'` separates pair chunks]
                    //    eg.
                    //      le'ss'ie -> le'sx'si'e
                    //      bu'rr'it'o -> bu'rx'ri'to
                    //    but:
                    //      ab'ba -> ab'ba
                    //      ir're'fu'ta'bl'e -> ir're'fu'ta'bl'e
                    // 2) Align word to even length (with 'x' as padding) (because we'll be operating on pairs of letters)
                    //    eg.
                    //      le'ss'ie -> le'sx'si'ex
                    //      ha'rb'ou'r -> ha'rb'ou'rx

                    // Edge case when word is one-letter long (eg. conjunction 'i' or article 'a')
                    if text.len() < 2 {
                        if text == "x" {
                            return Text::Word(format!("{}a", text));
                        }
                        return Text::Word(format!("{}x", text));
                    }

                    // Litte hack - we add additional 'x' as last letter
                    // because slice::windows() method returns None if there's no more values
                    // *or* length of remainder values is less than window size. That means in some
                    // cases (eg. ha'll'al -> ha'lx'la'[l]) we'd miss last letter. And btw. this
                    // guarantees we'll get even-aligned words.
                    let text: Vec<char> = text.chars().chain(core::iter::once('x')).collect();
                    let mut pairs = text.windows(2);

                    let mut text = String::with_capacity(text.len() + 1);

                    while let Some(pair) = pairs.next() {
                        match pair {
                            &[a, b] => {
                                if a == b {
                                    text.push(a);
                                    if a == 'x' {
                                        text.push('a');
                                    } else {
                                        text.push('x');
                                    }
                                } else {
                                    text.push(a);
                                    text.push(b);
                                    // if we didn't add anything we have to skip next window (because of windows overlapping).
                                    pairs.next();
                                }
                            }

                            // Slices can be only of length 2.
                            _ => unreachable!(),
                        }
                    }

                    Text::Word(text)
                }

                // We don't process punctuation, so let's just return it as is.
                punctuation => punctuation,
            }
        }

        #[cfg(test)]
        {
            let mut test_cases = vec![
                Text::Word("aabbaa".to_owned()),    // Expected to be "axabbaax"
                Text::Word("abba".to_owned()),      // Expected to be the same
                Text::Punctuation(", ".to_owned()), // Expected to be the same
                Text::Word("".to_owned()), // Incorrect - empty words are not allowed, however expected to be "x"
                Text::Word("i".to_owned()), // Expected to be "ix"
            ];

            let expected = [
                Text::Word("axabbaax".to_owned()),
                Text::Word("abba".to_owned()),
                Text::Punctuation(", ".to_owned()),
                Text::Word("x".to_owned()),
                Text::Word("ix".to_owned()),
            ];

            for (case, expect) in test_cases.drain(..).zip(expected) {
                let actual = process_word(case);
                assert_eq!(expect, actual);
            }
        }

        // Remove any non-ASCII-alphabetic or punctuation symbols.
        let message = deunicode(message);
        let msg: String = message
            .chars()
            .map(char::to_lowercase)
            .flatten()
            .filter(|letter| {
                [
                    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                    'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', ',', '.', ':', ';', '"',
                    '\'',
                ]
                .contains(letter)
            })
            .map(|letter| if letter == 'j' { 'i' } else { letter })
            .collect();

        enum State {
            Text,
            Punctuation,
        }

        let mut state = State::Text;
        let mut sanitized: Vec<Text> = Vec::with_capacity(message.len() / 5);
        let mut text = String::with_capacity(10);
        for letter in msg.chars() {
            match state {
                State::Text => {
                    if !letter.is_ascii_alphabetic() {
                        state = State::Punctuation;
                        sanitized.push(Text::Word(text.clone()));
                        text.clear();
                    }
                }
                State::Punctuation => {
                    if letter.is_ascii_alphabetic() {
                        state = State::Text;
                        sanitized.push(Text::Punctuation(text.clone()));
                        text.clear();
                    }
                }
            }
            text.push(letter);
        }
        // We have to push last word/punctuation into vector.
        match state {
            State::Text => sanitized.push(Text::Word(text)),
            State::Punctuation => sanitized.push(Text::Punctuation(text)),
        }

        sanitized.drain(..).map(process_word).collect()
    }

    pub fn new(keyword: &str) -> Result<Self, NonAsciiKey> {
        Ok(Self {
            key: Self::generate_key(keyword)?,
        })
    }

    pub fn encrypt(&self, message: &str) -> String {
        let sanitized = Self::sanitize_message(&message);
        let mut encrypted = String::with_capacity(message.len());

        for text in sanitized {
            match text {
                Text::Word(word) => {
                    for pair in word.chars().collect::<Vec<_>>().chunks(2) {
                        match pair {
                            [a, b] => {
                                let a = self
                                    .key
                                    .iter()
                                    .enumerate()
                                    .find(|&(_index, letter)| letter == a)
                                    .and_then(|(index, _)| Some(index))
                                    .expect("Unexpected non-ASCII character in message.");
                                let b = self
                                    .key
                                    .iter()
                                    .enumerate()
                                    .find(|&(_index, letter)| letter == b)
                                    .and_then(|(index, _)| Some(index))
                                    .expect("Unexpected non-ASCII character in message.");

                                let a_row = a / 5;
                                let a_col = a % 5;
                                let b_row = b / 5;
                                let b_col = b % 5;

                                let (a_idx, b_idx) = if a_row == b_row {
                                    let a_idx = a_row * 5 + (a_col + 1) % 5;
                                    let b_idx = b_row * 5 + (b_col + 1) % 5;
                                    (a_idx, b_idx)
                                } else if a_col == b_col {
                                    let a_idx = ((a_row + 1) % 5) * 5 + a_col;
                                    let b_idx = ((b_row + 1) % 5) * 5 + b_col;
                                    (a_idx, b_idx)
                                } else {
                                    let a_idx = b_row * 5 + a_col;
                                    let b_idx = a_row * 5 + b_col;
                                    (a_idx, b_idx)
                                };
                                encrypted.push(self.key[a_idx]);
                                encrypted.push(self.key[b_idx]);
                            }
                            _ => unreachable!("Unexpected word chunk size"),
                        }
                    }
                }
                Text::Punctuation(punctuation) => encrypted.push_str(&punctuation),
            }
        }

        encrypted
    }

    pub fn decrypt(&self, message: &str) -> String {
        let sanitized = Self::sanitize_message(&message);
        let mut decrypted = String::with_capacity(message.len());

        for text in sanitized {
            match text {
                Text::Word(word) => {
                    for pair in word.chars().collect::<Vec<_>>().chunks(2) {
                        match pair {
                            [a, b] => {
                                let a = self
                                    .key
                                    .iter()
                                    .enumerate()
                                    .find(|&(_index, letter)| letter == a)
                                    .and_then(|(index, _)| Some(index))
                                    .expect("Unexpected non-ASCII character in message.")
                                    as isize;
                                let b = self
                                    .key
                                    .iter()
                                    .enumerate()
                                    .find(|&(_index, letter)| letter == b)
                                    .and_then(|(index, _)| Some(index))
                                    .expect("Unexpected non-ASCII character in message.")
                                    as isize;

                                let a_row = a / 5;
                                let a_col = a % 5;
                                let b_row = b / 5;
                                let b_col = b % 5;

                                // rem_euclid is required, because default % operation for isize is n % d == n - (n / d) * d
                                // which results in negative indices.
                                let (a_idx, b_idx) = if a_row == b_row {
                                    let a_idx = a_row * 5 + (a_col - 1).rem_euclid(5);
                                    let b_idx = b_row * 5 + (b_col - 1).rem_euclid(5);
                                    (a_idx as usize, b_idx as usize)
                                } else if a_col == b_col {
                                    let a_idx = ((a_row - 1).rem_euclid(5)) * 5 + a_col;
                                    let b_idx = ((b_row - 1).rem_euclid(5)) * 5 + b_col;
                                    (a_idx as usize, b_idx as usize)
                                } else {
                                    let a_idx = b_row * 5 + a_col;
                                    let b_idx = a_row * 5 + b_col;
                                    (a_idx as usize, b_idx as usize)
                                };
                                decrypted.push(self.key[a_idx]);
                                decrypted.push(self.key[b_idx]);
                            }
                            _ => unreachable!("Unexpected word chunk size"),
                        }
                    }
                }
                Text::Punctuation(punctuation) => decrypted.push_str(&punctuation),
            }
        }

        decrypted
    }
}

#[cfg(test)]
mod test_playfair {
    use super::*;

    #[test]
    fn test_create_key() {
        let primary_key = "krypto";
        let expected = vec![
            'k', 'r', 'y', 'p', 't', 'o', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'l', 'm',
            'n', 'q', 's', 'u', 'v', 'w', 'x', 'z',
        ];
        let actual = Playfair::generate_key(primary_key).expect("Unexpected NonAsciiKey error.");

        assert_eq!(expected, actual);

        let primary_key = "kryptografia";
        let expected = vec![
            'k', 'r', 'y', 'p', 't', 'o', 'g', 'a', 'f', 'i', 'b', 'c', 'd', 'e', 'h', 'l', 'm',
            'n', 'q', 's', 'u', 'v', 'w', 'x', 'z',
        ];
        let actual = Playfair::generate_key(primary_key).expect("Unexpected NonAsciiKey error.");

        assert_eq!(expected, actual);

        let primary_key = "Juxtaposition";
        let expected = vec![
            'i', 'u', 'x', 't', 'a', 'p', 'o', 's', 'n', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'k',
            'l', 'm', 'q', 'r', 'v', 'w', 'y', 'z',
        ];
        let actual = Playfair::generate_key(primary_key).expect("Unexpected NonAsciiKey error.");

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn test_incorrect_keyword() {
        let _ = Playfair::generate_key("r??sum??").unwrap();
    }

    #[test]
    fn test_sanitize_message() {
        let message = "aabbaa kanna, xxx:";
        let expected = vec![
            Text::Word("axabbaax".to_owned()),
            Text::Punctuation(" ".to_owned()),
            Text::Word("kanxna".to_owned()),
            Text::Punctuation(", ".to_owned()),
            Text::Word("xxxxxx".to_owned()),
            Text::Punctuation(":".to_owned()),
        ];

        let actual = Playfair::sanitize_message(message);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_encrypt_message() {
        let encryptor = Playfair::new("kryptografia").expect("Unexpected non-ASCII keyword error.");
        let message = "Faramir caught Gollum";
        let encrypted = "ifgygsvp gdovsi aguqukvq";

        let actual = encryptor.encrypt(message);

        assert_eq!(encrypted, &actual);

        let message = "r??sum??";
        let encrypted = "cpzlcq";

        let actual = encryptor.encrypt(message);

        assert_eq!(encrypted, &actual);
    }

    #[test]
    fn test_decrypt_message() {
        let decryptor = Playfair::new("kryptografia").expect("Unexpected non-ASCII keyword error.");
        let encrypted = "ifgygsvp gdovsi aguqukvq";
        let decrypted = "faramirx caught golxlumx";

        let actual = decryptor.decrypt(encrypted);

        assert_eq!(decrypted, &actual);

        let encrypted = "cpzlcq";
        let decrypted = "resume";

        let actual = decryptor.decrypt(encrypted);

        assert_eq!(decrypted, &actual);
    }
}
