use std::env;
use syllabize_es::Word;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let word = &args[1];
    let res: Word = word.as_str().into();
    println!("{}", res.syllabify("-"));
}
