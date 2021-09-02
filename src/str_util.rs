use crate::char_util::IsVowel;

/// Returns true if the given string is a consonant blend or a digraph
pub fn is_consonant_group(s: &str) -> bool {
    matches!(
        s,
        "bl" | "fl"
            | "cl"
            | "gl"
            | "pl"
            | "cr"
            | "br"
            | "tr"
            | "gr"
            | "fr"
            | "pr"
            | "dr"
            | "tl"
            | "ch"
            | "ll"
            | "rr"
    )
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
        "a" | "á" => matches!(t, "a" | "á"),
        "e" | "é" => matches!(t, "e" | "é"),
        "i" | "í" => matches!(t, "i" | "í"),
        "o" | "ó" => matches!(t, "o" | "ó"),
        "u" | "ú" | "ü" => matches!(t, "u" | "ú" | "ü"),
        "ai" | "ay" => matches!(t, "ai" | "ay"),
        "oi" | "oy" => matches!(t, "oi" | "oy"),
        "ei" | "ey" => matches!(t, "ei" | "ey"),
        "ui" | "uy" | "üi" | "üy" => matches!(t, "ui" | "uy" | "üi" | "üy"),
        _ => false,
    }
}

pub fn is_both_s_or_z(a: &str, b: &str) -> bool {
    (a == "s" && b == "z") || (a == "z" && b == "s")
}

pub fn is_both_b_or_v(a: &str, b: &str) -> bool {
    a == "b" && b == "v" || a == "v" && b == "b"
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
