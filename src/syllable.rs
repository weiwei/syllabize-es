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
    /// Turn the syllable into a string.
    /// ```
    /// use syllabize_es::syllable::*;
    /// let syllable = Syllable {
    ///     onset: "b".to_string(),
    ///     nucleus: "üey".to_string(),
    ///     coda: "".to_string()
    /// };
    /// assert_eq!(syllable.to_string(), "büey");
    /// ```
    pub fn to_string(&self) -> String {
        let mut result = String::from(&self.onset);
        result.push_str(self.nucleus.as_str());
        result.push_str(self.coda.as_str());
        result
    }

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
        if tonic_vowels.chars().count() == 1 {
            tonic_vowels
        } else {
            let index = stress_index(tonic_vowels.as_str());
            tonic_vowels.chars().skip(index).collect::<String>()
        }
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
        assert_eq!(s.to_string(), "bat");
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
