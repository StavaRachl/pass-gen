use rand::{Rng, RngExt};

const NUMBERS: &str = "1234567890";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWZYX";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwzyx";
const SPECIAL: &str = "!@#$%&*+-=";

pub struct PasswordConfig {
    pub length: usize,
    pub include_upper: bool,
    pub include_lower: bool,
    pub include_special: bool,
    pub word: String
}

pub struct PasswordGenerator;
impl PasswordGenerator {
    pub fn generate<R: Rng + ?Sized> (config: &PasswordConfig, rng: &mut R) -> String {
        let alphabet = build_alphabet(config);

        let mut password = String::with_capacity(config.word.len() + config.length);

        password.push_str(&config.word);

        for _ in 0..config.length {
            let index = rng.random_range(0..alphabet.len());
            password.push(alphabet.as_bytes()[index] as char)
        }

        password
    }
}

fn build_alphabet(config: &PasswordConfig) -> String {
    let mut alphabet = String::from(NUMBERS);

    if config.include_lower {
        alphabet.push_str(LOWERCASE);
    }

    if config.include_upper {
        alphabet.push_str(UPPERCASE);
    }

    if config.include_special {
        alphabet.push_str(SPECIAL);
    }

    alphabet
}
