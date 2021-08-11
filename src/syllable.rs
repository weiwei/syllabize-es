use crate::char_util::*;
use crate::str_util::stress_index;

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
            if c.is_accented_vowel() {
                return true;
            }
        }
        false
    }

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
        assert_eq!(s.vowels_since_stress() , "ái");
    }
}