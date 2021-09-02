use crate::{DiphthongType, HiatusType};

pub trait IsVowel: private::Sealed {
    fn is_vowel(&self) -> bool;
    fn is_weak_vowel(&self) -> bool;
    fn is_stressed_vowel(&self) -> bool;
    fn is_accented_vowel(&self) -> bool;
    fn is_soft_c_trigger(&self) -> bool;
}

impl IsVowel for char {
    fn is_vowel(&self) -> bool {
        matches!(
            self,
            'a' | 'e'
                | 'i'
                | 'o'
                | 'u'
                | 'ü'
                | 'A'
                | 'E'
                | 'I'
                | 'O'
                | 'U'
                | 'Ü'
                | 'á'
                | 'é'
                | 'í'
                | 'ó'
                | 'ú'
                | 'Á'
                | 'É'
                | 'Í'
                | 'Ó'
                | 'Ú'
        )
    }
    fn is_weak_vowel(&self) -> bool {
        matches!(self, 'i' | 'y' | 'u' | 'ü' | 'I' | 'Y' | 'U' | 'Ü')
    }
    fn is_stressed_vowel(&self) -> bool {
        matches!(
            self,
            'á' | 'é'
                | 'í'
                | 'ó'
                | 'ú'
                | 'Á'
                | 'É'
                | 'Í'
                | 'Ó'
                | 'Ú'
                | 'a'
                | 'e'
                | 'o'
                | 'A'
                | 'E'
                | 'O'
        )
    }
    fn is_accented_vowel(&self) -> bool {
        matches!(
            self,
            'á' | 'é' | 'í' | 'ó' | 'ú' | 'Á' | 'É' | 'Í' | 'Ó' | 'Ú'
        )
    }

    fn is_soft_c_trigger(&self) -> bool {
        matches!(
            self,
            'e' | 'i' | 'é' | 'í' | 'É' | 'Í' | 'E' | 'I' | 'y' | 'Y'
        )
    }
}

mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for char {}
}

pub enum ComboType {
    Diphthong(DiphthongType),
    Hiatus(HiatusType),
    Other,
}

/// Returns true if two charactors make a hiatus.
///
/// A hiatus(hiato) is when two vowels are the same (with or without accent),
/// or both are stressed vowels.
pub fn combo_type(a: char, b: char) -> ComboType {
    match a {
        'e' | 'a' | 'E' | 'A' | 'á' | 'é' | 'Á' | 'É' => match b {
            'i' | 'u' | 'I' | 'U' => ComboType::Diphthong(DiphthongType::Falling),
            'a' | 'e' | 'o' | 'ü' | 'A' | 'E' | 'O' | 'Ü' => {
                ComboType::Hiatus(HiatusType::Simple)
            }
            'á' | 'é' | 'í' | 'ó' | 'ú' | 'Á' | 'É' | 'Í' | 'Ó' | 'Ú' => {
                ComboType::Hiatus(HiatusType::Accentual)
            }
            _ => ComboType::Other,
        },
        'i' | 'I' => match b {
            'a' | 'e' | 'o' | 'á' | 'é' | 'ó' | 'A' | 'E' | 'O' | 'Á' | 'É' | 'Ó' => {
                ComboType::Diphthong(DiphthongType::Rising)
            }
            'u' | 'ú' | 'ü' | 'U' | 'Ú' | 'Ü' => {
                ComboType::Diphthong(DiphthongType::Homogenous)
            }
            'i' | 'I' => ComboType::Hiatus(HiatusType::Simple),
            'í' | 'Í' => ComboType::Hiatus(HiatusType::Accentual),
            _ => ComboType::Other,
        },
        'o' | 'O' => match b {
            'i' | 'u' | 'ü' | 'I' | 'U' | 'Ü' => ComboType::Diphthong(DiphthongType::Falling),
            'a' | 'e' | 'o' | 'A' | 'E' | 'O' => ComboType::Hiatus(HiatusType::Simple),
            'á' | 'é' | 'í' | 'ó' | 'ú' | 'Á' | 'É' | 'Í' | 'Ó' | 'Ú' => {
                ComboType::Hiatus(HiatusType::Accentual)
            }
            _ => ComboType::Other,
        },
        'u' | 'ü' | 'U' | 'Ü' => match b {
            'a' | 'e' | 'o' | 'á' | 'é' | 'ó' | 'A' | 'E' | 'O' | 'Á' | 'É' | 'Ó' => {
                ComboType::Diphthong(DiphthongType::Rising)
            }
            'i' | 'I' => ComboType::Diphthong(DiphthongType::Homogenous),
            'u' | 'ü' | 'U' | 'Ü' => ComboType::Hiatus(HiatusType::Simple),
            'ú' | 'Ú' => ComboType::Hiatus(HiatusType::Accentual),
            _ => ComboType::Other,
        },
        'í' | 'Í' | 'ú' | 'Ú' => match b {
            'a' | 'e' | 'i' | 'u' | 'o' | 'A' | 'E' | 'I' | 'U' | 'O' => {
                ComboType::Hiatus(HiatusType::Simple)
            }
            'á' | 'é' | 'í' | 'ó' | 'ú' | 'ü' | 'Á' | 'É' | 'Í' | 'Ó' | 'Ú' | 'Ü' => {
                ComboType::Hiatus(HiatusType::Accentual)
            }
            _ => ComboType::Other,
        },
        'ó' | 'Ó' => match b {
            'i' | 'u' | 'ü' | 'I' | 'U' | 'Ü' => ComboType::Diphthong(DiphthongType::Falling),
            'a' | 'e' | 'o' | 'á' | 'é' | 'í' | 'ó' | 'ú' | 'A' | 'E' | 'O' | 'Á' | 'É' | 'Í'
            | 'Ó' | 'Ú' => ComboType::Hiatus(HiatusType::Accentual),
            _ => ComboType::Other,
        },
        _ => ComboType::Other,
    }
}

pub fn can_form_hiatus(a: char, b: char) -> bool {
    matches!(combo_type(a, b), ComboType::Hiatus(_))
}

/// Returns true if three characters make a triphthong.
pub fn can_form_triphthong(a: char, b: char, c: char) -> bool {
    a.is_weak_vowel() && b.is_stressed_vowel() && c.is_weak_vowel()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_is_triphthong() {
        assert_eq!(can_form_triphthong('a', 'b', 'c'), false);
        assert_eq!(can_form_triphthong('i', 'e', 'i'), true);
        assert_eq!(can_form_triphthong('i', 'á', 'i'), true);
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

    #[bench]
    fn lower2(b: &mut Bencher) {
        b.iter(|| 'a'.is_vowel())
    }
}
