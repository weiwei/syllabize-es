#![feature(test)]

extern crate test;

use regex::Regex;
use std::usize;
use test::Bencher;

pub mod char_util;
pub mod str_util;
pub mod syllable;

use crate::str_util::{is_consonant_group, stress_index};
use crate::char_util::is_hiatus;
use crate::char_util::is_triphthong;
use crate::char_util::IsVowel;
use crate::syllable::Syllable;

#[derive(PartialEq, Debug)]
pub enum StressType {
    Oxytone,            // aguda
    Paroxytone,         // llana or grave
    Proparoxytone,      // esdrújula
    Superproparoxytone, //sobresdrújula
}

// TODO:
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

enum HiatusType {
    Simple,
    Acentual,
}

enum DiphthongType {
    Rising,     // Creciente
    Falling,    // Decrescente
    Homogenous, // Homogéneo o Anticreciente
}

pub struct Hiatus {
    syllable_index: usize,
    composite: String,
    kind: HiatusType,
}

pub struct Diphthong {
    syllable_index: usize,
    composite: String,
    kind: DiphthongType,
}

pub struct Triphthong {
    syllable_index: usize,
    composite: String,
}

#[derive(PartialEq, Debug)]
pub struct Stress {
    pub kind: StressType,
    pub index: usize,
}

pub struct VowelCombos {
    pub hiatuses: Vec<Hiatus>,
    pub diphthongs: Vec<Diphthong>,
    pub triphthongs: Vec<Triphthong>,
}

#[derive()]
pub struct Word {
    pub syllables: Vec<Syllable>,
    pub stress: Stress,
}

impl Word {
    pub fn rhyme(&self) -> String {
        let stress_syllable = &self.syllables[self.stress.index];
        let tonic_vowels = stress_syllable.nucleus.chars().collect::<String>();
        let mut rhyme = stress_syllable.vowels_since_stress();
        rhyme.push_str(stress_syllable.coda.as_str());

        for i in self.stress.index + 1..self.syllables.len() {
            rhyme.push_str(&self.syllables[i].to_string().as_str())
        }
        rhyme
    }

    pub fn vowel_combos(&self) -> VowelCombos {
        let syllables = &self.syllables;
        let mut index = 0;
        let mut hiatuses = vec![];
        let mut diphthongs = vec![];
        let mut triphthongs = vec![];
        let dp_rising = Regex::new("[iíuúü][aáeéoó]").unwrap();
        let dp_falling = Regex::new("[aáeéoó][iíuúüy]").unwrap();
        let dp_homogenous = Regex::new("[iíuúü][iíuúüy]").unwrap();
        while index < syllables.len() {
            if syllables[index].coda == ""
                && syllables[index].nucleus.chars().count() == 1
                && index + 1 < syllables.len()
                && (syllables[index + 1].onset == "" || syllables[index + 1].onset == "h")
                && syllables[index + 1].nucleus.chars().count() == 1
            {
                let mut composite = syllables[index].nucleus.clone();
                composite.push_str(syllables[index + 1].nucleus.as_str());
                hiatuses.push(Hiatus {
                    syllable_index: index,
                    composite,
                    kind: if syllables[index].has_accent() || syllables[index + 1].has_accent() {
                        HiatusType::Acentual
                    } else {
                        HiatusType::Simple
                    },
                });
            } else if syllables[index].nucleus.chars().count() == 2 {
                let dp_type: DiphthongType;
                if dp_rising.is_match(syllables[index].nucleus.to_lowercase().as_str()) {
                    dp_type = DiphthongType::Rising;
                } else if dp_falling.is_match(syllables[index].nucleus.to_lowercase().as_str()) {
                    dp_type = DiphthongType::Falling;
                } else if dp_homogenous.is_match(syllables[index].nucleus.to_lowercase().as_str()) {
                    dp_type = DiphthongType::Homogenous;
                } else {
                    panic!("Not a diphthong");
                }
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
            } else if syllables[index].coda == ""
                && syllables[index].nucleus.chars().count() == 2
                && index + 1 < syllables.len()
                && (syllables[index + 1].onset == "" || syllables[index + 1].onset == "h")
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

    pub fn syllabify(&self, delimiter: &str) -> String {
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
        let stress = identify_stress(&syllables);
        Word { syllables, stress }
    }
}

fn to_syllables(word: &str) -> Vec<Syllable> {
    let mut index = 0;
    let mut position = Position::None;
    let mut syllable = Syllable {
        onset: "".to_string(),
        nucleus: "".to_string(),
        coda: "".to_string(),
    };
    let chars: Vec<char> = word.chars().collect();
    let word_len = chars.len();
    let mut syllables: Vec<Syllable> = Vec::new();
    loop {
        let curr_char = chars[index];
        if !curr_char.is_vowel() {
            if position == Position::None || position == Position::Onset {
                position = Position::Onset;
                syllable.onset.push(curr_char);
            } else if position == Position::Nucleus {
                if curr_char == 'y'
                    && (index == word_len - 1
                        || (index + 1 < word_len && !chars[index + 1].is_vowel()))
                {
                    syllable.nucleus.push(curr_char);
                } else if curr_char == 'h' {
                    index += 1;
                    let next_char = chars[index];
                    if next_char.is_vowel() {
                        if syllable.nucleus.chars().count() == 1
                            && is_hiatus(syllable.nucleus.chars().nth(0).unwrap(), next_char)
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
                            let after_next_char = chars[index];
                            if after_next_char.is_vowel() {
                                if is_triphthong(
                                    syllable.nucleus.chars().nth(0).unwrap(),
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
                } else {
                    syllable.coda.push(curr_char);
                    position = Position::Coda;
                }
            } else if position == Position::Coda {
                syllable.coda.push(curr_char);
            }
        } else {
            if position == Position::None || position == Position::Onset {
                position = Position::Nucleus;
                syllable.nucleus.push(curr_char);
            } else if position == Position::Nucleus {
                if syllable.nucleus.chars().count() == 1 {
                    if is_hiatus(curr_char, syllable.nucleus.chars().nth(0).unwrap()) {
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
                    if is_triphthong(
                        syllable.nucleus.chars().nth(0).unwrap(),
                        syllable.nucleus.chars().nth(1).unwrap(),
                        curr_char,
                    ) {
                        syllable.nucleus.push(curr_char);
                        position = Position::Nucleus;
                    } else {
                        let last_nucleus = syllable.nucleus.chars().nth(1).unwrap();
                        if last_nucleus.is_weak_vowel() {
                            syllable.nucleus = syllable.nucleus.chars().nth(0).unwrap().to_string();
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
                        position = Position::Nucleus;
                    }
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
                        syllable.coda = syllable.coda.chars().nth(0).unwrap().to_string();
                    }
                    syllables.push(syllable);
                    syllable = Syllable {
                        onset: temp,
                        nucleus: curr_char.to_string(),
                        coda: "".to_string(),
                    };
                } else if syllable.coda.chars().count() == 3 {
                    let temp = syllable.coda.as_str()[1..3].to_string();
                    syllable.coda = syllable.coda.chars().nth(0).unwrap().to_string();
                    syllables.push(syllable);
                    syllable = Syllable {
                        onset: temp,
                        nucleus: curr_char.to_string(),
                        coda: "".to_string(),
                    }
                } else if syllable.coda.chars().count() == 4 {
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
        }
        index += 1;
        if index > word_len - 1 {
            syllables.push(syllable);
            break;
        }
    }
    syllables
}

fn identify_stress(syllables: &Vec<Syllable>) -> Stress {
    let syllable_count = syllables.len();
    if syllable_count == 1 {
        return Stress {
            kind: StressType::Oxytone,
            index: 0,
        };
    }
    if syllable_count > 1 && syllables[syllable_count - 1].has_accent() {
        return Stress {
            kind: StressType::Oxytone,
            index: syllable_count - 1,
        };
    }
    if syllable_count >= 2 && syllables[syllable_count - 2].has_accent() {
        return Stress {
            kind: StressType::Paroxytone,
            index: syllable_count - 2,
        };
    }
    if syllable_count >= 3 && syllables[syllable_count - 3].has_accent() {
        return Stress {
            kind: StressType::Proparoxytone,
            index: syllable_count - 3,
        };
    }
    if syllable_count >= 4 {
        let mut index = syllable_count as i32 - 4;
        while index >= 0 {
            if syllables[index as usize].has_accent() {
                return Stress {
                    kind: StressType::Superproparoxytone,
                    index: index as usize,
                };
            }
            index -= 1;
        }
    }

    let last_coda = syllables[syllable_count - 1].coda.as_str();
    if last_coda != "" && last_coda != "n" && last_coda != "s" {
        return Stress {
            kind: StressType::Oxytone,
            index: syllable_count - 1,
        };
    }

    return Stress {
        kind: StressType::Paroxytone,
        index: syllable_count - 2,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let word: Word = "palabra".into();
        assert_eq!(word.syllables[0].to_string(), "pa");
        assert_eq!(word.syllables[1].to_string(), "la");
        assert_eq!(word.syllables[2].to_string(), "bra");
        assert_eq!(
            word.stress,
            Stress {
                kind: StressType::Paroxytone,
                index: 1
            }
        );
        let vowel_combos = word.vowel_combos();
        assert_eq!(vowel_combos.hiatuses.len(), 0);
        assert_eq!(vowel_combos.diphthongs.len(), 0);
        assert_eq!(vowel_combos.triphthongs.len(), 0);
        assert_eq!(word.rhyme(), "abra".to_string());
    }

    #[test]
    fn test_hiato() {
        let word: Word = "lee".into();
        let vowel_combos = word.vowel_combos();
        assert_eq!(vowel_combos.hiatuses.len(), 1);
        assert_eq!(vowel_combos.diphthongs.len(), 0);
        assert_eq!(vowel_combos.triphthongs.len(), 0);
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
        b.iter(|| {
            word.vowel_combos()
        });
    }

    #[bench]
    fn bench_rhyme(b: &mut Bencher) {
        let word: Word = "envergadura".into();
        b.iter(|| {
            word.rhyme()
        });
    }

    #[bench]
    fn bench_syllabify(b: &mut Bencher) {
        let word: Word = "envergadura".into();
        b.iter(|| {
            word.syllabify("-")
        });
    }
}
