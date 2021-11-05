use std::fs;

pub struct Util {}

#[allow(dead_code)]
impl Util {
    const ALPHABET: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    fn get_position(letter: char) -> usize {
        Util::ALPHABET.iter().position(|&c| c == letter).unwrap()
    }
    fn get_letter(position: usize) -> char {
        Util::ALPHABET[position % Util::ALPHABET.len()]
    }

    pub fn load_file(path: &str) -> String {
        fs::read_to_string(path).unwrap().parse().unwrap()
    }
    pub fn vigenere_cipher(text: &str, key: &str) -> String {
        let lower_text = text.to_lowercase();
        let key = key.to_lowercase();

        let mut i: usize = 0;
        let mut res = String::new();
        for c in lower_text.chars() {
            if Util::ALPHABET.contains(&c) {
                let shift = Util::get_position(key.chars().nth(i).unwrap());
                res.push(Util::get_letter(Util::get_position(c) + shift + 1));
                i = (i + 1) % key.chars().count();
            }
        }
        res
    }
}
