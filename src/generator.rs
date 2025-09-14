use rand::{rng, seq::IndexedMutRandom};

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.<>?/";

pub fn generate_password(length: usize, use_symbols: bool) -> String {
    let mut charset = LETTERS.chars().collect::<Vec<char>>();
    if use_symbols {
        charset.extend(SYMBOLS.chars());
    }

    let mut rng = rng();
    (0..length)
        .map(|_| *charset.choose_mut(&mut rng).unwrap())
        .collect()
}
