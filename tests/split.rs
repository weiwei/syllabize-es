use syllabize_es::Word;
// use regex::Regex;

#[test]
fn diptongos() {
    let data = vec![
        "pai-sa-je",
        "pei-ne",
        "an-droi-de",
        "pau-sa",
        "feu-do",
        "es-ta-dou-ni-den-se",
        "su-cia",
        "tie-rra",
        "pio-jo",
        "re-cua",
        "puer-ta",
        "re-si-duo",
        "ciu-dad",
        "bui-tre",
        "muy",
        "viu-da",
        "pié",
    ];
    let src = data
        .iter()
        .map(|s| (s.replace("-", ""), s.to_string()))
        .collect::<Vec<(String, String)>>();
    for (s, d) in src {
        let w: Word = s.as_str().into();
        assert_eq!(w.syllabize("-"), d);
    }
}

#[test]
fn hiatos() {
    let data = vec![
        // e
        "le-e",
        "pa-se-é",
        "se-a",
        "te-a-tro",
        "cre-ó",
        "fe-o",
        "re-í",
        "re-ú-ne",
        // a
        "ca-e",
        "a-é-re-o",
        "a-za-har",
        "na-o",
        "ca-o-ba",
        "pa-ís",
        "ba-úl",
        // o
        "ro-e",
        "No-é",
        "bo-a-to",
        "Sa-mo-a",
        "lo-ó",
        "zo-o",
        "o-ír",
        "No-ú-me-no",
        // i
        "rí-e",
        "fi-lo-so-fí-a",
        "rí-o",
        "chi-i-ta",
        // u
        "li-cú-e",
        "pú-a",
        "a-cen-tú-o",
        "du-un-vi-ro",
    ];
    let src = data
        .iter()
        .map(|s| (s.replace("-", ""), s.to_string()))
        .collect::<Vec<(String, String)>>();
    for (s, d) in src {
        let w: Word = s.as_str().into();
        assert_eq!(w.syllabize("-"), d);
    }
}

// TODO: oa, ea no son hiatos en Mexico

#[test]
fn triptongos() {
    let data = vec![
        "A-bre-viáis",
        "A-bre-viéis",
        "Ac-tuáis",
        "A-huau-tle",
        "Buey",
        "Ca-ma-güey",
        "Ciais",
        "Crieis",
        "Cuai-mas",
        "Dioi-co",
        "Guay-mas",
        "Huey-pox-tla",
        "Miau",
    ];
    let src = data
        .iter()
        .map(|s| (s.replace("-", ""), s.to_string()))
        .collect::<Vec<(String, String)>>();
    for (s, d) in src {
        let w: Word = s.as_str().into();
        assert_eq!(w.syllabize("-"), d);
    }
}

#[test]
fn no_triptongos() {
    let data = vec!["lim-pia-ú-ñas", "vi-ví-ais"];
    let src = data
        .iter()
        .map(|s| (s.replace("-", ""), s.to_string()))
        .collect::<Vec<(String, String)>>();
    for (s, d) in src {
        let w: Word = s.as_str().into();
        assert_eq!(w.syllabize("-"), d);
    }
}

#[test]
fn ad_hoc() {
    // Most from https://github.com/vic/silabas.js
    let data = vec![
        "a",
        "va-te",
        "su-yo",
        "ár-bol",
        "pa-la-bra",
        "es-to-cól-mo",
        "i-dí-li-co",
        "i-rru-ma-ción",
        "i-ne-fa-ble",
        "hi-po-pó-ta-mo",
        "ay",
        "hay",
        "ma-guey",
        "a-buha-do",
        "ac-mé",
        "hai-ga",
        "mam-po-rre-ro",
        "mur-cié-la-go",
        "ple-io-tro-pí-a",
        "Ab-yec-ción",
        "A-he-rro-jar",
        "güe-ro",
        "a-ve-ri-guáis",
        "U-ru-guay",
        "huí-a",
        "az-ca-pot-zal-co",
        "va-he-e",
        "pte-ra-sau-rio",
        "por-que",
        "abs-tra-er",
        "cons-truir",
        "ads-cri-bir",
        "ads-trin-gir",
        "ah-re",
        "e-lec-tro-en-ce-fa-lo-gra-fis-ta",
    ];
    let src = data
        .iter()
        .map(|s| (s.replace("-", ""), s.to_string()))
        .collect::<Vec<(String, String)>>();
    for (s, d) in src {
        let w: Word = s.as_str().into();
        assert_eq!(w.syllabize("-"), d);
    }
}

#[test]
fn with_h() {
    let data = vec![
        "a-ni-hi-lar", 
        "ma-ri-hua-na", 
        "vih",  // not a word but oh well
        "ahi",  // wrong spelling but oh well
        "a-hí"
        ];
    let src = data
        .iter()
        .map(|s| (s.replace("-", ""), s.to_string()))
        .collect::<Vec<(String, String)>>();
    for (s, d) in src {
        let w: Word = s.as_str().into();
        assert_eq!(w.syllabize("-"), d);
    }
}
