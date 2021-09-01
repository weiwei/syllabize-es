use syllabize_es::{syllable::Syllable, Word};

#[test]
fn y() {
    let w: Word = "y".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "".to_string(),
            nucleus: "y".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn cc() {
    let w: Word = "nn".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "nn".to_string(),
            nucleus: "".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn vc() {
    let w: Word = "la".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "l".to_string(),
            nucleus: "a".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn cv() {
    let w: Word = "al".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "".to_string(),
            nucleus: "a".to_string(),
            coda: "l".to_string()
        }]
    );
}

#[test]
fn cvy() {
    let w: Word = "doy".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "d".to_string(),
            nucleus: "oy".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn vy() {
    let w: Word = "ey".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "".to_string(),
            nucleus: "ey".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn cvv() {
    let w: Word = "duo".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "d".to_string(),
            nucleus: "uo".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn cvvy() {
    let w: Word = "buey".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "b".to_string(),
            nucleus: "uey".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn gü() {
    let w: Word = "güey".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "g".to_string(),
            nucleus: "üey".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn gui() {
    let w: Word = "guitarra".into();
    assert_eq!(
        w.syllables,
        vec![
            Syllable {
                onset: "gu".to_string(),
                nucleus: "i".to_string(),
                coda: "".to_string()
            },
            Syllable {
                onset: "t".to_string(),
                nucleus: "a".to_string(),
                coda: "".to_string()
            },
            Syllable {
                onset: "rr".to_string(),
                nucleus: "a".to_string(),
                coda: "".to_string()
            }
        ]
    );
}

#[test]
fn guia() {
    let w: Word = "guiada".into();
    assert_eq!(
        w.syllables,
        vec![
            Syllable {
                onset: "gu".to_string(),
                nucleus: "ia".to_string(),
                coda: "".to_string()
            },
            Syllable {
                onset: "d".to_string(),
                nucleus: "a".to_string(),
                coda: "".to_string()
            }
        ]
    );
}

#[test]
fn que() {
    let w: Word = "que".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "qu".to_string(),
            nucleus: "e".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn rry() {
    let w: Word = "curry".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "c".to_string(),
            nucleus: "u".to_string(),
            coda: "".to_string()
        },Syllable {
            onset: "rr".to_string(),
            nucleus: "y".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn nry() {
    let w: Word = "henry".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "h".to_string(),
            nucleus: "e".to_string(),
            coda: "n".to_string()
        },Syllable {
            onset: "r".to_string(),
            nucleus: "y".to_string(),
            coda: "".to_string()
        }]
    );
}

#[test]
fn vg() {
    let w: Word = "blog".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "bl".to_string(),
            nucleus: "o".to_string(),
            coda: "g".to_string()
        }]
    );
}

#[test]
fn vng() {
    let w: Word = "hong".into();
    assert_eq!(
        w.syllables,
        vec![Syllable {
            onset: "h".to_string(),
            nucleus: "o".to_string(),
            coda: "ng".to_string()
        }]
    );
}
