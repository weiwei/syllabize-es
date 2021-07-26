// #![feature(test)]
// #![feature(core::iter::traits::collect::Extend)]

pub const ACCENTED_VOWELS: &str = "áéíóúÁÉÍÓÚ";
const NON_ACCENTED_VOWELS: &str = "aeiouüAEIOUÜ";
const STRESSED_VOWELS: &str = "áéíóúaoe";
const WEAK_VOWELS: &str = "iuüIUÜ";
const CONSONANT_BLENDS: &[&str] = &[
    "bl", "fl", "cl", "gl", "pl", "cr", "br", "tr", "gr", "fr", "pr", "dr", "tl",
];

const DIAGRAPHS: &[&str] = &["ch", "ll", "rr"];

pub trait IsVowel: private::Sealed {
    fn is_vowel(self) -> bool;
    fn is_weak_vowel(self) -> bool;
    fn is_stressed_vowel(self) -> bool;
}

impl IsVowel for char {
    fn is_vowel(self) -> bool {
        ACCENTED_VOWELS.contains(self) || NON_ACCENTED_VOWELS.contains(self)
    }
    fn is_weak_vowel(self) -> bool {
        WEAK_VOWELS.contains(self)
    }
    fn is_stressed_vowel(self) -> bool {
        STRESSED_VOWELS.contains(self)
    }
}

mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for char {}
}

pub fn is_consonant_group(part: &str) -> bool {
    CONSONANT_BLENDS.contains(&part) || DIAGRAPHS.contains(&part)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Syllable {
    pub onset: String,
    pub nucleus: String,
    pub coda: String,
}

impl Syllable {
    pub fn to_string(&self) -> String {
        let mut result = String::from(&self.onset);
        result.push_str(self.nucleus.as_str());
        result.push_str(self.coda.as_str());
        result
    }
    pub fn has_accent(&self) -> bool {
        for c in self.nucleus.chars().into_iter() {
            if ACCENTED_VOWELS.contains(c) {
                return true;
            }
        }
        false
    }
}

pub fn stress_index(part: &str) -> i32 {
    let mut index = 0;
    for ch in part.chars() {
        if STRESSED_VOWELS.contains(ch) {
            break;
        }
        index += 1;
    }
    index
}

const VOWEL_A: &str = "aáAÁ";
const VOWEL_E: &str = "eéEÉ";
const VOWEL_I: &str = "iíIÍ";
const VOWEL_O: &str = "oóOÓ";
const VOWEL_U: &str = "uúüUÚÜ";

pub fn is_hiatus(a: char, b: char) -> bool {
    VOWEL_A.contains(a) && VOWEL_A.contains(b)
        || VOWEL_E.contains(a) && VOWEL_E.contains(b)
        || VOWEL_I.contains(a) && VOWEL_I.contains(b)
        || VOWEL_O.contains(a) && VOWEL_O.contains(b)
        || VOWEL_U.contains(a) && VOWEL_U.contains(b)
        || STRESSED_VOWELS.contains(a) && STRESSED_VOWELS.contains(b)
}

pub fn is_triphthong(a: char, b: char, c: char) -> bool {
    a.is_weak_vowel() && b.is_stressed_vowel() && c.is_weak_vowel()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triphthong() {
        assert_eq!(is_triphthong('a', 'b', 'c'), false);
        assert_eq!(is_triphthong('i', 'e', 'i'), true);
    }

    #[test]
    fn test_is_hiatus() {
        assert_eq!(is_hiatus('a', 'b'), false);
        assert_eq!(is_hiatus('a', 'e'), true);
    }

    #[test]
    fn test_is_vowel() {
        assert_eq!('e'.is_vowel(), true);
        assert_eq!('f'.is_vowel(), false);
    }

    #[test]
    fn to_string_works() {
        let s = Syllable {
            onset: "b".to_string(),
            nucleus: "a".to_string(),
            coda: "t".to_string(),
        };
        assert_eq!(s.to_string(), "bat");
    }
}
