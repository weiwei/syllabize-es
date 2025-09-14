// #![warn(missing_docs)]
#![feature(test)]

//! Converting Spanish words into syllables, and much more!

extern crate test;

use char_util::can_form_hiatus;
use std::fmt;
use std::fmt::Display;
use std::usize;
use str_util::is_both_b_or_v;
use str_util::is_both_s_or_z;
use str_util::loose_match;

pub mod char_util;
pub mod str_util;
pub mod syllable;

use crate::char_util::can_form_triphthong;
use crate::char_util::combo_type;
use crate::char_util::ComboType;
use crate::char_util::IsVowel;
use crate::str_util::is_consonant_group;
use crate::syllable::Syllable;

type Result<T> = std::result::Result<T, InvalidWord>;

#[derive(Debug, Clone)]
struct InvalidWord;

impl fmt::Display for InvalidWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid word")
    }
}

/// Types of stress
#[derive(PartialEq, Debug)]
pub enum StressType {
    /// aguda, stress at the last syllable
    Oxytone,
    /// llana or grave, stress at the penultimate syllable
    Paroxytone,
    /// esdrújula, stress at the 3rd last syllable
    Proparoxytone,
    /// sobresdrújula, stress at the 4th last syllable or earlier
    Superproparoxytone,
}

// TODO: Another stress type
// pub enum StressType {
//     Prosodic,  // Acento prosódico
//     Orthographic,  // Acento ortográfico
// }

#[derive(PartialEq)]
enum Position {
    None,
    Onset,   // Dos, ataque
    Nucleus, // dOs, nucleo
    Coda,    // doS
}

#[derive(PartialEq, Debug)]
pub enum HiatusType {
    Simple,
    Accentual,
}

#[derive(PartialEq, Debug)]
pub enum DiphthongType {
    Rising,     // Creciente
    Falling,    // Decrescente
    Homogenous, // Homogéneo o Anticreciente
}

pub struct Hiatus {
    pub syllable_index: usize,
    pub composite: String,
    pub kind: HiatusType,
}

pub struct Diphthong {
    pub syllable_index: usize,
    pub composite: String,
    pub kind: DiphthongType,
}

pub struct Triphthong {
    pub syllable_index: usize,
    pub composite: String,
}

pub struct VowelCombos {
    pub hiatuses: Vec<Hiatus>,
    pub diphthongs: Vec<Diphthong>,
    pub triphthongs: Vec<Triphthong>,
}

#[derive(Clone, Copy)]
pub struct RhymeOptions {
    pub yeismo: bool,
    pub seseo: bool,
    pub b_equals_v: bool,
}

pub fn equal_onset(a: &Syllable, b: &Syllable, opt: &RhymeOptions) -> bool {
    if a.onset == b.onset {
        return true;
    }
    if opt.seseo {
        if a.onset == "c" && b.onset == "s" {
            let a_first_nucleus = a.nucleus.chars().next().unwrap();
            if a_first_nucleus.is_soft_c_trigger()
                && loose_match(a.nucleus.as_str(), b.nucleus.as_str())
            {
                return true;
            }
        } else if a.onset == "s" && b.onset == "c" {
            let b_first_nucleus = b.nucleus.chars().next().unwrap();
            if b_first_nucleus.is_soft_c_trigger()
                && loose_match(a.nucleus.as_str(), b.nucleus.as_str())
            {
                return true;
            }
        } else if is_both_s_or_z(a.onset.as_str(), b.onset.as_str()) {
            return true;
        }
    }
    if opt.yeismo {
        if a.onset == "y" && b.onset == "ll" || a.onset == "ll" && b.onset == "y" {
            return true;
        }
    }
    if opt.b_equals_v {
        if is_both_b_or_v(a.onset.as_str(), b.onset.as_str()) {
            return true;
        }
    }

    false
}

/// A parsed word that contains syllables and stress information
#[derive(Clone, Debug)]
pub struct Word {
    pub syllables: Vec<Syllable>,
    pub stress_index: usize,
}

impl Word {
    pub fn rhyme(&self) -> String {
        if self.syllables.len() == 0 {
            return String::new();
        }
        let stress_syllable = &self.syllables[self.stress_index];
        let mut rhyme = stress_syllable.vowels_since_stress();
        rhyme.push_str(stress_syllable.coda.as_str());

        for i in self.stress_index + 1..self.syllables.len() {
            rhyme.push_str(self.syllables[i].to_string().as_str())
        }
        rhyme
    }

    pub fn assonant_rhymes_with(&self, other: &Word) -> bool {
        let this_syllables = &self.syllables[self.stress_index..self.syllables.len()];
        let that_syllables = &other.syllables[other.stress_index..other.syllables.len()];
        if this_syllables.len() != that_syllables.len() {
            return false;
        }
        for (i, j) in this_syllables.iter().enumerate() {
            if i == 0 {
                let k1 = j.vowels_since_stress();
                let k2 = that_syllables[i].vowels_since_stress();
                if k1 != k2 {
                    if k1.chars().count() == k2.chars().count() {
                        if k1.chars().count() == 1 {
                            let m1 = k1.chars().collect::<String>();
                            let m2 = k2.chars().collect::<String>();
                            if !loose_match(m1.as_str(), m2.as_str()) {
                                return false;
                            }
                        } else if k1.chars().collect::<String>() != k2.chars().collect::<String>() {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            } else if j.nucleus != that_syllables[i].nucleus {
                return false;
            }
        }
        true
    }

    pub fn rhymes_with(&self, other: &Word, opt: Option<RhymeOptions>) -> bool {
        let opt = opt.unwrap_or(RhymeOptions {
            seseo: false,
            yeismo: true,
            b_equals_v: true,
        });
        let this_syllables = &self.syllables[self.stress_index..self.syllables.len()];
        let that_syllables = &other.syllables[other.stress_index..other.syllables.len()];
        if this_syllables.len() != that_syllables.len() {
            return false;
        }
        for (i, j) in this_syllables.iter().enumerate() {
            if i == 0 {
                let k1 = j.vowels_since_stress();
                let k2 = that_syllables[i].vowels_since_stress();
                if this_syllables.len() == 1 {
                    if !loose_match(&k1, &k2) {
                        return false;
                    }
                    if !opt.seseo && j.coda != that_syllables[i].coda {
                        return false;
                    }
                    if opt.seseo && is_both_s_or_z(j.coda.as_str(), that_syllables[i].coda.as_str())
                    {
                        return true;
                    }
                    return true;
                } else if k1 != k2 || j.coda != that_syllables[i].coda {
                    return false;
                }
            } else if !equal_onset(j, &that_syllables[i], &opt) // j.onset != that_syllables[i].onset
                || j.nucleus != that_syllables[i].nucleus
                || !(j.coda == that_syllables[i].coda || (opt.seseo && is_both_s_or_z(j.coda.as_str(), that_syllables[i].coda.as_str())))
            {
                return false;
            }
        }
        true
    }

    pub fn vowel_combos(&self) -> VowelCombos {
        let syllables = &self.syllables;
        let mut index = 0;
        let mut hiatuses = vec![];
        let mut diphthongs = vec![];
        let mut triphthongs = vec![];
        while index < syllables.len() {
            let nucleus_chars: Vec<char> = syllables[index].nucleus.chars().collect();
            if syllables[index].coda.is_empty()
                && nucleus_chars.len() == 1
                && index + 1 < syllables.len()
                && (syllables[index + 1].onset.is_empty() || syllables[index + 1].onset == "h")
                && syllables[index + 1].nucleus.chars().count() == 1
            {
                let mut composite = syllables[index].nucleus.clone();
                composite.push_str(syllables[index + 1].nucleus.as_str());
                hiatuses.push(Hiatus {
                    syllable_index: index,
                    composite,
                    kind: if syllables[index].has_accented_vowel()
                        || syllables[index + 1].has_accented_vowel()
                    {
                        HiatusType::Accentual
                    } else {
                        HiatusType::Simple
                    },
                });
            } else if nucleus_chars.len() == 2 {
                let dp_type: DiphthongType = match combo_type(nucleus_chars[0], nucleus_chars[1]) {
                    ComboType::Diphthong(t) => t,
                    _ => panic!("Not a diphthong"),
                };
                diphthongs.push(Diphthong {
                    syllable_index: index,
                    kind: dp_type,
                    composite: syllables[index].nucleus.clone(),
                });
            } else if syllables[index].nucleus.chars().count() == 3 {
                triphthongs.push(Triphthong {
                    syllable_index: index,
                    composite: syllables[index].nucleus.clone(),
                });
            } else if syllables[index].coda.is_empty()
                && syllables[index].nucleus.chars().count() == 2
                && index + 1 < syllables.len()
                && (syllables[index + 1].onset.is_empty() || syllables[index + 1].onset == "h")
                && syllables[index + 1].nucleus.chars().count() == 1
            {
                // ???
            }
            index += 1;
        }
        VowelCombos {
            hiatuses,
            diphthongs,
            triphthongs,
        }
    }

    pub fn stress(&self) -> StressType {
        let d = self.syllables.len() - 1 - self.stress_index;
        match d {
            0 => StressType::Oxytone,
            1 => StressType::Paroxytone,
            2 => StressType::Proparoxytone,
            3.. => StressType::Superproparoxytone,
            _ => panic!("Invalid stress count {}", d),
        }
    }

    pub fn syllabize(&self, delimiter: &str) -> String {
        return self
            .syllables
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(delimiter);
    }
}

impl From<&str> for Word {
    fn from(item: &str) -> Self {
        let syllables = to_syllables(item);
        match syllables {
            Ok(s) => {
                let stress_index = identify_stress(&s);
                Word {
                    syllables: s,
                    stress_index,
                }
            }
            Err(_e) => Word {
                syllables: vec![],
                stress_index: 0,
            }
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = self
            .syllables
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", res)
    }
}



fn to_syllables(word: &str) -> Result<Vec<Syllable>> {
    if word.is_empty() {
        return Ok(vec![]);
    }

    let chars: Vec<char> = word.chars().collect();
    let word_len = chars.len();

    if word_len == 1 {
        return Ok(vec![Syllable {
            onset: "".to_string(),
            nucleus: chars[0].to_string(),
            coda: "".to_string(),
        }]);
    }

    // Officially the longest word is 12 syllables, here we give some leeway
    // shaves of 100ns
    let mut syllables: Vec<Syllable> = Vec::with_capacity(32);

    let mut index = 0;
    let mut position = Position::None;
    let mut syllable = Syllable {
        onset: "".to_string(),
        nucleus: "".to_string(),
        coda: "".to_string(),
    };

    loop {
        let curr_char = chars[index];
        if !curr_char.is_vowel() {
            if position == Position::None || position == Position::Onset {
                if curr_char == 'y' {
                    if syllable.onset.is_empty() {
                        syllable.onset.push(curr_char);
                        position = Position::Onset;
                    } else {
                        syllable.nucleus.push(curr_char);
                        position = Position::Nucleus
                    }
                } else if curr_char == 'q' || curr_char == 'g' {
                    syllable.onset.push(curr_char);
                    position = Position::Onset;
                    index += 1;
                    if word_len <= index {
                        return Err(InvalidWord)
                    }
                    let next_char = chars[index];
                    if next_char == 'u' {
                        index += 1;
                        if word_len <= index {
                            return Err(InvalidWord)
                        }
                        let after_next_char = chars[index];
                        if after_next_char == 'i' || after_next_char == 'e' {
                            syllable.onset.push(next_char);
                            syllable.nucleus.push(after_next_char);
                            position = Position::Nucleus;
                        }
                    } else {
                        index -= 1;
                    }
                } else {
                    syllable.onset.push(curr_char);
                    position = Position::Onset;
                }
            } else if position == Position::Nucleus {
                if curr_char == 'y'
                    && (index == word_len - 1
                        || (index + 1 < word_len && !chars[index + 1].is_vowel()))
                {
                    syllable.nucleus.push(curr_char);
                } else if curr_char == 'h' {
                    index += 1;
                    if word_len <= index {
                        return Err(InvalidWord)
                    }
                    if index == chars.len() {
                        syllable.coda.push(curr_char);
                        position = Position::Coda;
                    } else {
                        let next_char = chars[index];
                        if next_char.is_vowel() {
                            if syllable.nucleus.chars().count() == 1
                                && can_form_hiatus(
                                    syllable.nucleus.chars().next().unwrap(),
                                    next_char,
                                )
                            {
                                syllables.push(syllable);
                                syllable = Syllable {
                                    onset: curr_char.to_string(),
                                    nucleus: next_char.to_string(),
                                    coda: "".to_string(),
                                };
                                position = Position::Nucleus;
                            } else {
                                index += 1;
                                if index == chars.len() {
                                    syllable.nucleus.push(curr_char);
                                    syllable.nucleus.push(next_char);
                                    position = Position::Coda;
                                } else {
                                    let after_next_char = chars[index];
                                    if after_next_char.is_vowel() {
                                        if can_form_triphthong(
                                            syllable.nucleus.chars().next().unwrap(),
                                            next_char,
                                            after_next_char,
                                        ) {
                                            // Could this happen?
                                        } else {
                                            syllables.push(syllable);
                                            let mut nucleus = next_char.to_string();
                                            nucleus.push(after_next_char);
                                            syllable = Syllable {
                                                onset: curr_char.to_string(),
                                                nucleus,
                                                coda: "".to_string(),
                                            };
                                            position = Position::Nucleus;
                                        }
                                    } else {
                                        syllable.nucleus.push(curr_char);
                                        syllable.nucleus.push(next_char);
                                        syllable.coda.push(after_next_char);
                                        position = Position::Coda;
                                    }
                                }
                            }
                        } else {
                            syllable.coda.push(curr_char);
                            syllables.push(syllable);
                            syllable = Syllable {
                                onset: next_char.to_string(),
                                nucleus: "".to_string(),
                                coda: "".to_string(),
                            };
                            position = Position::Onset;
                        }
                    }
                } else {
                    syllable.coda.push(curr_char);
                    position = Position::Coda;
                }
            } else if position == Position::Coda {
                if curr_char == 'y' {
                    if syllable.coda.chars().count() == 1 {
                        if index + 1 < word_len {
                            if chars[index + 1].is_vowel() {
                                syllables.push(syllable);
                                syllable = Syllable {
                                    onset: curr_char.to_string(),
                                    nucleus: "".to_string(),
                                    coda: "".to_string(),
                                };
                                position = Position::Onset;
                            } else {
                                let onset = syllable.coda.clone();
                                syllable.coda.clear();
                                syllables.push(syllable);
                                syllable = Syllable {
                                    onset,
                                    nucleus: curr_char.to_string(),
                                    coda: "".to_string(),
                                };
                                position = Position::Nucleus;
                            }
                        }
                    } else if syllable.coda.chars().count() == 2 {
                        if is_consonant_group(syllable.coda.as_str()) {
                            let onset = syllable.coda.clone();
                            syllable.coda = "".to_string();
                            syllables.push(syllable);
                            syllable = Syllable {
                                onset,
                                nucleus: curr_char.to_string(),
                                coda: "".to_string(),
                            };
                            position = Position::Nucleus;
                        } else {
                            let chars: Vec<char> = syllable.coda.chars().collect();
                            let onset = chars[1].to_string();
                            syllable.coda = chars[0].to_string();
                            syllables.push(syllable);
                            syllable = Syllable {
                                onset,
                                nucleus: curr_char.to_string(),
                                coda: "".to_string(),
                            };
                            position = Position::Nucleus;
                        }
                    } else {
                        syllable.coda.push(curr_char);
                    }
                } else {
                    syllable.coda.push(curr_char);
                }
            }
        } else if position == Position::None || position == Position::Onset {
            position = Position::Nucleus;
            syllable.nucleus.push(curr_char);
        } else if position == Position::Nucleus {
            if syllable.nucleus.chars().count() == 1 {
                if can_form_hiatus(syllable.nucleus.chars().next().unwrap(), curr_char) {
                    syllables.push(syllable);
                    syllable = Syllable {
                        onset: "".to_string(),
                        nucleus: curr_char.to_string(),
                        coda: "".to_string(),
                    };
                } else {
                    syllable.nucleus.push(curr_char);
                }
            } else if syllable.nucleus.chars().count() == 2 {
                if can_form_triphthong(
                    syllable.nucleus.chars().next().unwrap(),
                    syllable.nucleus.chars().nth(1).unwrap(),
                    curr_char,
                ) {
                    syllable.nucleus.push(curr_char);
                } else {
                    let last_nucleus = syllable.nucleus.chars().nth(1).unwrap();
                    if last_nucleus.is_weak_vowel() {
                        syllable.nucleus = syllable.nucleus.chars().next().unwrap().to_string();
                        syllables.push(syllable);
                        let mut last_nucleus = last_nucleus.to_string();
                        last_nucleus.push(curr_char);
                        syllable = Syllable {
                            onset: "".to_string(),
                            nucleus: last_nucleus,
                            coda: "".to_string(),
                        }
                    } else {
                        syllables.push(syllable);
                        syllable = Syllable {
                            onset: "".to_string(),
                            nucleus: curr_char.to_string(),
                            coda: "".to_string(),
                        }
                    }
                }
                position = Position::Nucleus;
            }
        } else if position == Position::Coda {
            if syllable.coda.chars().count() == 1 {
                let temp = syllable.coda.clone();
                syllable.coda = "".to_string();
                syllables.push(syllable);
                syllable = Syllable {
                    onset: temp,
                    nucleus: curr_char.to_string(),
                    coda: "".to_string(),
                }
            } else if syllable.coda.chars().count() == 2 {
                let temp: String;
                if is_consonant_group(syllable.coda.as_str()) {
                    temp = syllable.coda.clone();
                    syllable.coda = "".to_string();
                } else {
                    temp = syllable.coda.chars().nth(1).unwrap().to_string();
                    syllable.coda = syllable.coda.chars().next().unwrap().to_string();
                }
                syllables.push(syllable);
                syllable = Syllable {
                    onset: temp,
                    nucleus: curr_char.to_string(),
                    coda: "".to_string(),
                };
            } else if syllable.coda.chars().count() == 3 {
                let temp = syllable.coda.chars().skip(1).collect::<String>();
                syllable.coda = syllable.coda.chars().next().unwrap().to_string();
                syllables.push(syllable);
                syllable = Syllable {
                    onset: temp,
                    nucleus: curr_char.to_string(),
                    coda: "".to_string(),
                }
            } else if syllable.coda.chars().count() == 4 {
                // indexing into &str should be fine because 4 char consonant
                // clusters would only contain ascii letters, i.e., no `ñ`.
                let temp = syllable.coda.as_str()[2..4].to_string();
                syllable.coda = syllable.coda.as_str()[0..2].to_string();
                syllables.push(syllable);
                syllable = Syllable {
                    onset: temp,
                    nucleus: curr_char.to_string(),
                    coda: "".to_string(),
                }
            }
            position = Position::Nucleus;
        }

        index += 1;
        if index > word_len - 1 {
            syllables.push(syllable);
            break;
        }
    }
    Ok(syllables)
}

fn identify_stress(syllables: &[Syllable]) -> usize {
    let syllable_count = syllables.len();
    if syllable_count == 0 || syllable_count == 1 {
        return 0;
    }
    if syllable_count > 1 && syllables[syllable_count - 1].has_accented_vowel() {
        return syllable_count - 1;
    }
    if syllable_count >= 2 && syllables[syllable_count - 2].has_accented_vowel() {
        return syllable_count - 2;
    }
    if syllable_count >= 3 && syllables[syllable_count - 3].has_accented_vowel() {
        return syllable_count - 3;
    }
    if syllable_count >= 4 {
        let mut index = syllable_count as i8 - 4;
        while index >= 0 {
            if syllables[index as usize].has_accented_vowel() {
                return index as usize;
            }
            index -= 1;
        }
    }

    let last_syllable = &syllables[syllable_count - 1];
    if last_syllable.coda.is_empty() {
        if last_syllable.nucleus.chars().count() == 3 {
            return syllable_count - 1;
        }
    } else {
        if last_syllable.coda != "n" && last_syllable.coda != "s" {
            return syllable_count - 1;
        }
    }

    syllable_count - 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_hiato() {
        let word: Word = "lee".into();
        let vowel_combos = word.vowel_combos();
        assert_eq!(vowel_combos.hiatuses.len(), 1);
        assert_eq!(vowel_combos.diphthongs.len(), 0);
        assert_eq!(vowel_combos.triphthongs.len(), 0);
    }

    #[test]
    fn test_rhymes_with() {
        let word: Word = "vida".into();
        assert!(word.rhymes_with(&Word::from("frida"), None));
        assert!(!word.rhymes_with(&Word::from("vía"), None));
        assert!(word.assonant_rhymes_with(&Word::from("villa")));
        assert!(word.assonant_rhymes_with(&Word::from("vía")));
    }

    #[test]
    fn test_rhymes_with_yeismo() {
        let word: Word = "callo".into();
        assert!(word.rhymes_with(
            &Word::from("cayo"),
            Some(RhymeOptions {
                yeismo: true,
                seseo: false,
                b_equals_v: true
            })
        ));
        assert!(!word.rhymes_with(
            &Word::from("cayo"),
            Some(RhymeOptions {
                yeismo: false,
                seseo: false,
                b_equals_v: true
            })
        ));
    }

    #[test]
    fn test_rhymes_with_seseo() {
        let word: Word = "cabeza".into();
        assert!(word.rhymes_with(
            &Word::from("besa"),
            Some(RhymeOptions {
                yeismo: true,
                seseo: true,
                b_equals_v: true
            })
        ));
        assert!(!word.rhymes_with(
            &Word::from("besa"),
            Some(RhymeOptions {
                yeismo: true,
                seseo: false,
                b_equals_v: true
            })
        ));
    }

    #[test]
    fn test_rhymes_with_seseo_end() {
        let word: Word = "estrés".into();
        assert!(word.rhymes_with(
            &Word::from("vez"),
            Some(RhymeOptions {
                yeismo: true,
                seseo: true,
                b_equals_v: true
            })
        ));
        assert!(!word.rhymes_with(
            &Word::from("vez"),
            Some(RhymeOptions {
                yeismo: true,
                seseo: false,
                b_equals_v: true
            })
        ));
    }

    #[test]
    fn test_rhymes_b_v() {
        let word: Word = "escribe".into();
        assert!(word.rhymes_with(
            &Word::from("declive"),
            Some(RhymeOptions {
                yeismo: true,
                seseo: true,
                b_equals_v: true
            })
        ));
        assert!(!word.rhymes_with(
            &Word::from("declive"),
            Some(RhymeOptions {
                yeismo: true,
                seseo: true,
                b_equals_v: false
            })
        ));
    }

    #[test]
    fn test_rhymes_with_single_syllable() {
        let word: Word = "vi".into();
        assert!(word.rhymes_with(&Word::from("tí"), None));
    }

    #[test]
    fn test_rhymes_with_y() {
        let word: Word = "ley".into();
        assert!(!word.rhymes_with(&Word::from("é"), None));
    }

    #[test]
    fn test_rhymes_no_match() {
        let word: Word = "vi".into();
        assert!(!word.rhymes_with(&Word::from("sid"), None));
    }

    #[test]
    fn test_diptongo() {
        let word: Word = "DIALOGO".into();
        let vowel_combos = word.vowel_combos();
        assert_eq!(vowel_combos.hiatuses.len(), 0);
        assert_eq!(vowel_combos.diphthongs.len(), 1);
        assert_eq!(vowel_combos.triphthongs.len(), 0);
    }

    #[bench]
    fn bench_wordify(b: &mut Bencher) {
        b.iter(|| {
            let _word: Word = "envergadura".into();
        });
    }

    #[bench]
    fn bench_vowel_combos(b: &mut Bencher) {
        let word: Word = "envergadura".into();
        b.iter(|| word.vowel_combos());
    }

    #[bench]
    fn bench_rhyme(b: &mut Bencher) {
        let word: Word = "envergadura".into();
        b.iter(|| word.rhyme());
    }

    #[bench]
    fn bench_syllabize(b: &mut Bencher) {
        let word: Word = "envergadura".into();
        b.iter(|| word.syllabize("-"));
    }
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            unsafe extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
