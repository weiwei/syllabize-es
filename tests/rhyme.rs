use syllabize_es::Word;

#[test]
fn agudo() {
    let word: Word = "pié".into();
    assert_eq!(word.rhyme, "é");
}

#[test]
fn llano() {
    let word: Word = "ciento".into();
    assert_eq!(word.rhyme, "ento");
}

#[test]
fn esdrújulo() {
    let word: Word = "esdrújulo".into();
    assert_eq!(word.rhyme, "újulo");
}
