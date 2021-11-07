use std::fs;

pub struct Util {}

#[allow(dead_code)]
impl Util {
    const ALPHABET: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    /// get position by letter in the alphabet
    fn get_position(letter: char) -> usize {
        Util::ALPHABET.iter().position(|&c| c == letter).unwrap()
    }
    /// get a letter by its position in the alphabet
    fn get_letter(position: usize) -> char {
        Util::ALPHABET[position % Util::ALPHABET.len()]
    }

    /// laod file with a given path
    pub fn load_file(path: &str) -> String {
        fs::read_to_string(path).unwrap().parse().unwrap()
    }
    /// apply Vigenere cipher to text
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

#[cfg(test)]
mod tests {
    use crate::Util;

    #[test]
    fn test_get_position() {
        let comp = 0;
        let ret = Util::get_position('a');

        assert_eq!(comp, ret);
    }
    #[test]
    fn test_get_letter() {
        let comp = 'a';
        let ret = Util::get_letter(0);

        assert_eq!(comp, ret);
    }
    #[test]
    fn test_vigenere_cipher() {
        let comp = "bdf";
        let ret = Util::vigenere_cipher("abc", "abc");

        assert_eq!(comp, ret);
    }
}
