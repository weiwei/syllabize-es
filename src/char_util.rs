const ACCENTED_VOWELS: &str = "áéíóúÁÉÍÓÚ";
const NON_ACCENTED_VOWELS: &str = "aeiouüAEIOUÜ";
const STRESSED_VOWELS: &str = "áéíóúaoeÁÉÍÓÚAOE";
const WEAK_VOWELS: &str = "iuüIUÜ";

pub trait IsVowel: private::Sealed {
    fn is_vowel(self) -> bool;
    fn is_weak_vowel(self) -> bool;
    fn is_stressed_vowel(self) -> bool;
    fn is_accented_vowel(self) -> bool;
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
    fn is_accented_vowel(self) -> bool {
        ACCENTED_VOWELS.contains(self)
    }
}

mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for char {}
}

const VOWEL_A: &str = "aáAÁ";
const VOWEL_E: &str = "eéEÉ";
const VOWEL_I: &str = "iíIÍ";
const VOWEL_O: &str = "oóOÓ";
const VOWEL_U: &str = "uúüUÚÜ";

/// Returns true if two charactors make a hiatus.
///
/// A hiatus(hiato) is when two vowels are the same (with or without accent),
/// or both are stressed vowels.
pub fn can_form_hiatus(a: char, b: char) -> bool {
    VOWEL_A.contains(a) && VOWEL_A.contains(b)
        || VOWEL_E.contains(a) && VOWEL_E.contains(b)
        || VOWEL_I.contains(a) && VOWEL_I.contains(b)
        || VOWEL_O.contains(a) && VOWEL_O.contains(b)
        || VOWEL_U.contains(a) && VOWEL_U.contains(b)
        || STRESSED_VOWELS.contains(a) && STRESSED_VOWELS.contains(b)
}

/// Returns true if three charactors make a triphthong.
pub fn can_form_triphthong(a: char, b: char, c: char) -> bool {
    a.is_weak_vowel() && b.is_stressed_vowel() && c.is_weak_vowel()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triphthong() {
        assert_eq!(can_form_triphthong('a', 'b', 'c'), false);
        assert_eq!(can_form_triphthong('i', 'e', 'i'), true);
    }

    #[test]
    fn test_is_hiatus() {
        assert_eq!(can_form_hiatus('a', 'b'), false);
        assert_eq!(can_form_hiatus('a', 'e'), true);
    }

    #[test]
    fn test_is_vowel() {
        assert_eq!('e'.is_vowel(), true);
        assert_eq!('f'.is_vowel(), false);
    }
}
