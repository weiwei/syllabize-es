# syllabize-es

Turns Spanish words into syllables, and much more.

## Example

```rust
use syllabize_es::{syllable::Syllable, Word, StressType, DiphthongType};

// Convert a word into syllabized struct
let word: Word = "construir".into();

// Number of syllables
assert_eq!(word.syllables.len(), 2);

// First syllable, in string form
assert_eq!(word.syllables[0].to_string(), "cons");

// Second syllable, in struct form
assert_eq!(
    word.syllables[1],
    Syllable {
        onset: "tr".to_string(),
        nucleus: "ui".to_string(),
        coda: "r".to_string()
    }
);

// Get syllabified string, using "-" as delimiter
assert_eq!(word.syllabize("-"), "cons-truir");

// Index of the stressed syllable of `word.syllables`
assert_eq!(word.stress_index, 1);

// Named type of the stress
assert_eq!(word.stress(), StressType::Oxytone);

// All existing vowel combinations
let vowel_combos = word.vowel_combos();

// The word doesn't contain hiatuses or triphthongs
assert_eq!(vowel_combos.hiatuses.len(), 0);
assert_eq!(vowel_combos.triphthongs.len(), 0);

// But it contains a diphthong
assert_eq!(vowel_combos.diphthongs.len(), 1);

let dp = &vowel_combos.diphthongs[0];
// All its attributes
assert_eq!(dp.syllable_index, 1);
assert_eq!(dp.kind, DiphthongType::Homogenous);
assert_eq!(dp.composite, "ui");

// The rhyming part of the word
assert_eq!(word.rhyme(), "ir");
```

A simple command line utility is also provided:

```shell-session
$ syllabize palabra
pa-la-bra
```