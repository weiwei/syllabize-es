use std::fmt::{self, Display};

use crate::char_util::*;
use crate::str_util::stress_index;

/// Basic syllable and its related methods.
#[derive(Debug, Clone, PartialEq)]
pub struct Syllable {
    /// The starting consonant(s). "ataque" in Spanish.
    pub onset: String,
    /// The vowel(s), "nucleo" in Spanish.
    pub nucleus: String,
    /// Finishing consonant(s). "coda" in Spanish.
    pub coda: String,
}

impl Syllable {
    /// Returns true if there is an accented vowel in the nucleus.
    ///
    /// # Example
    ///
    /// ```
    /// use syllabize_es::syllable::*;
    /// let syllable = Syllable {
    ///     onset: "b".to_string(),
    ///     nucleus: "üey".to_string(),
    ///     coda: "".to_string()
    /// };
    /// assert_eq!(syllable.has_accented_vowel(), false);
    /// ```
    pub fn has_accented_vowel(&self) -> bool {
        for c in self.nucleus.chars().into_iter() {
            if c.is_accented_vowel() {
                return true;
            }
        }
        false
    }

    /// Returns part of the nucleus starting from the stressed vowel.
    /// ```
    /// use syllabize_es::syllable::*;
    /// let syllable = Syllable {
    ///     onset: "b".to_string(),
    ///     nucleus: "üey".to_string(),
    ///     coda: "".to_string()
    /// };
    /// assert_eq!(syllable.vowels_since_stress(), "ey");
    /// ```
    pub fn vowels_since_stress(&self) -> String {
        let tonic_vowels = self.nucleus.chars().collect::<String>();
        match tonic_vowels.chars().count() {
            0 => return "".to_string(),
            1 => return tonic_vowels,
            _ => {
                let index = stress_index(tonic_vowels.as_str());
                tonic_vowels.chars().skip(index).collect::<String>()
            }
        }
    }
}

impl Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", &self.onset, &self.nucleus, &self.coda)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        let s = Syllable {
            onset: "b".to_string(),
            nucleus: "a".to_string(),
            coda: "t".to_string(),
        };
        assert_eq!(format!("{}", s), "bat");
    }

    #[test]
    fn vowels_since_stress() {
        let s = Syllable {
            onset: "t".to_string(),
            nucleus: "uái".to_string(),
            coda: "s".to_string(),
        };
        assert_eq!(s.vowels_since_stress(), "ái");
    }
}
