use std::env;

use silabize_es::Word;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let word = &args[1];
    let res: Word = word.as_str().into();
    let strings: Vec<String> = res.syllables.into_iter().map(|s| s.to_string()).collect();
    println!("{}", strings.join(", "));
}
