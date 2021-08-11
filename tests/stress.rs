use syllabize_es::{StressType, Word};

#[test]
fn test_oxytone() {
    let data = vec![
        "a", "la", "gol", "olé", "pié", "piedad", "pastel", "habló", "reloj", "vivir",
    ];
    for s in data {
        let w: Word = s.into();
        assert_eq!(w.stress.0, StressType::Oxytone);
    }
}

#[test]
fn test_paroxytone() {
    let data = vec![
        "pena",
        "gases",
        "ponen",
        "lee",
        "cóctel",
        "espantoso",
        "bíceps",
        "fértil",
    ];
    for s in data {
        let w: Word = s.into();
        assert_eq!(w.stress.0, StressType::Paroxytone);
    }
}

#[test]
fn test_proparoxytones() {
    let data = vec!["esdrújula", "teléfono", "árboles"];
    for s in data {
        let w: Word = s.into();
        assert_eq!(w.stress.0, StressType::Proparoxytone);
    }
}

#[test]
fn test_superproparoxytone() {
    let data = vec!["tráiganosla", "gíratelo", "rápidamente", "ávidamente"];
    for s in data {
        let w: Word = s.into();
        assert_eq!(w.stress.0, StressType::Superproparoxytone);
    }
}
