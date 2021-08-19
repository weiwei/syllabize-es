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
