fn main() {
    // Ouch
    let exclamation = input("Exclamation: ");
    // stupidly
    let adverb = input("Adverb: ");
    // car
    let noun = input("Noun: ");
    // brave
    let adjective = input("Adjective: ");

    let madlib = format!(
        "{}! he said {} as he jumped into his convertible {} and drove off with his {} wife",
        exclamation, adverb, noun, adjective
    );

    println!("{}", madlib);
}

fn input(message: &str) -> String {
    use std::io::{self, Write};
    use text_io::read;

    print!("{}", message);
    io::stdout().flush().unwrap();
    read!("{}\n")
}
