use crate::char_util::{IsVowel};

/// Returns true if the given string is a consonant blend or a digraph
pub fn is_consonant_group(s: &str) -> bool {
    match s {
        "bl"| "fl"| "cl"| "gl"| "pl"| "cr"| "br"| "tr"| "gr"| "fr"| "pr"| "dr"| "tl" => true,
        "ch"| "ll"| "rr" => true,
        _ => false,
    }
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
    match s {
        "a" | "á" => {
            match t {
                "a" | "á" => true,
                _ => false,
            }
        },
        "e" | "é" => {
            match t {
                "e" | "é" => true,
                _ => false,
            }
        },
        "i" | "í" => {
            match t {
                "i" | "í" => true,
                _ => false,
            }
        },
        "o" | "ó" => {
            match t {
                "o" | "ó" => true,
                _ => false,
            }
        },
        "u" | "ú" | "ü" => {
            match t {
                "u" | "ú" | "ü" => true,
                _ => false,
            }
        },
        "ai" | "ay" => {
            match t {
                "ai" | "ay" => true,
                _ => false,
            }
        },
        "oi" | "oy" => {
            match t {
                "oi" | "oy" => true,
                _ => false,
            }
        },
        "ei" | "ey" => {
            match t {
                "ei" | "ey" => true,
                _ => false,
            }
        },
        "ui" | "uy" | "üi" | "üy" => {
            match t {
                "ui" | "uy" | "üi" | "üy" => true,
                _ => false,
            }
        },
        _ => false,
    }
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
