use crate::char_util::IsVowel;

const CONSONANT_BLENDS: &[&str] = &[
    "bl", "fl", "cl", "gl", "pl", "cr", "br", "tr", "gr", "fr", "pr", "dr", "tl",
];

const DIAGRAPHS: &[&str] = &["ch", "ll", "rr"];

pub fn is_consonant_group(part: &str) -> bool {
    CONSONANT_BLENDS.contains(&part) || DIAGRAPHS.contains(&part)
}

pub fn stress_index(part: &str) -> usize {
    let mut index = 0;
    for ch in part.chars() {
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
