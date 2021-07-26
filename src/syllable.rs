use crate::char_util::ACCENTED_VOWELS;
use std::string::String;

#[derive(Debug)]
struct Syllable {
    onset: String,
    nucleus: String,
    coda: String,
}

impl Syllable {
    fn has_accent(&self) -> bool {
        for ch in self.nucleus.chars() {
            if ACCENTED_VOWELS.contains(ch) {
                return true;
            }
        }
        return false;
    }

    fn is_open(&self) -> bool {
        self.coda.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_accent() {
        let s = Syllable {
            onset: String::from("f"),
            nucleus: String::from("ee"),
            coda: String::from("t"),
        };
        assert_eq!(s.has_accent(), false);
        let t = Syllable {
            onset: String::from("f"),
            nucleus: String::from("eá"),
            coda: String::from("t"),
        };
        assert_eq!(t.has_accent(), true);
    }
}
