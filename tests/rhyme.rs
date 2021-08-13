use syllabize_es::Word;

#[test]
fn agudo() {
    let word: Word = "pié".into();
    assert_eq!(word.rhyme(), "é");
}

#[test]
fn corta() {
    let word: Word = "y".into();
    assert_eq!(word.rhyme(), "y");
}

#[test]
fn invalida() {
    let word: Word = "nn".into();
    assert_eq!(word.rhyme(), "");
}

#[test]
fn agudo2() {
    let word: Word = "huir".into();
    assert_eq!(word.rhyme(), "ir");
}

#[test]
fn llano() {
    let word: Word = "ciento".into();
    assert_eq!(word.rhyme(), "ento");
}

#[test]
fn esdrújulo() {
    let word: Word = "esdrújulo".into();
    assert_eq!(word.rhyme(), "újulo");
}
