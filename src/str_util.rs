use crate::char_util::IsVowel;

const CONSONANT_BLENDS: &[&str] = &[
    "bl", "fl", "cl", "gl", "pl", "cr", "br", "tr", "gr", "fr", "pr", "dr", "tl",
];

const DIAGRAPHS: &[&str] = &["ch", "ll", "rr"];

/// @brief Returns true if the given string is a consonant blend or a digraph
/// @param s The string to test
/// @return True if the string is a consonant blend or digraph
/// @see CONSONANT_BLENDS
/// @see DIAGRAPHS
pub fn is_consonant_group(s: &str) -> bool {
    CONSONANT_BLENDS.contains(&s) || DIAGRAPHS.contains(&s)
}

/// @brief Returns index of the stressed vowel in the given string
/// @param s The string to test
/// @return Index of the stressed vowel in the string
/// @see is_stressed_vowel
pub fn stress_index(s: &str) -> usize {
    let mut index = 0;
    for ch in s.chars() {
        if ch.is_stressed_vowel() {
            break;
        }
        index += 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_consonant_group() {
        assert!(is_consonant_group("cl"));
        assert!(!is_consonant_group("cy"));
    }
}
