# syllabize-es

Turns Spanish words into syllables, and much more.

## Intro

The package tries to duplicate most of the functions from [Silabeador TIP](https://tulengua.es/silabas/) and provide some more:

* Dividing words into syllables.
* Identify the stress.
* Find diphthongs, triphthongs, and hiatuses, and provide detailed info about them.
* Finding the rhyming part of a word.

It is tested against a comprehensive dataset, so the package should be quite reliable.

## Example

```rust
use syllabize_es::{syllable::Syllable, Word, StressType, DiphthongType, RhymeType};

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

// The rhyming part of the word
assert!(word.rhymes_with(&Word::from("ir"), RhymeType::Consonant));
assert!(word.rhymes_with(&Word::from("colibrí"), RhymeType::Assonant));
```

## CLI Example

```shell-session
$ syllabize palabra
pa-la-bra
```

## Limitations and TODOs

Some attributes aren't provided because they are trivial to tell, without syllabizing a word:

* Length of a word.
* Tipo de acento: prosódico(has no accent mark) o ortográfico(has an accent mark). 

Some attributes aren't provided because it's trivial to get. For example: Tonic syllable is easy to get when you have a vector of syllables and the index of the stressed syllable.

Some features may be provided in future releases, for example:

* Pronombre enclítico. te, lo, etc. at the trailing of a word.

## License

MIT.

## Inspirations

The package is a rewriting of the NPM package [silabacion](https://www.npmjs.com/package/silabacion), but with a simpler interface.
