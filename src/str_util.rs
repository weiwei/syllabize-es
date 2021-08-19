use regex::Regex;

use crate::char_util::IsVowel;

const CONSONANT_BLENDS: &[&str] = &[
    "bl", "fl", "cl", "gl", "pl", "cr", "br", "tr", "gr", "fr", "pr", "dr", "tl",
];

const DIAGRAPHS: &[&str] = &["ch", "ll", "rr"];

/// Returns true if the given string is a consonant blend or a digraph
pub fn is_consonant_group(s: &str) -> bool {
    CONSONANT_BLENDS.contains(&s) || DIAGRAPHS.contains(&s)
}

/// Returns index of the stressed vowel in the given string of vowels
pub fn stress_index(s: &str) -> usize {
    let mut index = 0;
    for ch in s.chars() {
        if ch.is_stressed_vowel() {
            break;
        }
        index += 1;
    }
    if index == s.len() {
        // No stressed vowel, take the last, eg. "i" of "ui".
        index - 1
    } else {
        index
    }
}

pub fn loose_match(s: &str, t: &str) -> bool {
    let regexes = vec![
        Regex::new("^[áa]$").unwrap(),
        Regex::new("^[ée]$").unwrap(),
        Regex::new("^[íi]$").unwrap(),
        Regex::new("^[óo]$").unwrap(),
        Regex::new("^[úuü]$").unwrap(),
        Regex::new("^a[iy]$").unwrap(),
        Regex::new("^o[iy]$").unwrap(),
        Regex::new("^e[iy]$").unwrap(),
        Regex::new("^[uü][iy]$").unwrap(),
    ];

    let mut matched = false;

    for re in regexes {
        if re.is_match(s) && re.is_match(t) {
            matched = true;
            break;
        }
    }
    matched
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_consonant_group() {
        assert!(is_consonant_group("cl"));
        assert!(!is_consonant_group("cy"));
    }

    #[test]
    fn test_stress_index() {
        assert_eq!(stress_index("ui"), 1);
        assert_eq!(stress_index("ai"), 0);
    }

    #[test]
    fn test_loose_match() {
        assert!(loose_match("i", "í"));
        assert!(!loose_match("ey", "é"));
    }
}
