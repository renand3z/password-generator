use colored::Colorize;
use names::{Generator, Name};

const PREFIX: &str = "1A!";

// get a pair of words
pub fn get_word_pair(n: u8) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();

    for _ in 0..n {
        let generator = Generator::with_naming(Name::Plain).next().unwrap();
        let mut collected = generator.split("-");
        words.push(collected.next().unwrap().to_string());
        words.push(collected.next().unwrap().to_string());
    }

    return words;
}

#[test]
fn print_words() {
    print_colored(4);
}

// create a password string
pub fn format_password(n: u8) -> String {
    let words: Vec<String> = Vec::from_iter(get_word_pair(n));
    let mut sentence: String = words.join("");
    sentence = PREFIX.to_owned() + &sentence;
    return sentence;
}

// create a password and print on the terminal with colors
pub fn print_colored(n: u8) {
    let words: Vec<String> = Vec::from_iter(get_word_pair(n));

    print!("{}", PREFIX);

    for i in 0..words.len() {
        if i % 2 == 0 {
            print!("{}", words[i].cyan());
        } else {
            print!("{}", words[i].red());
        }
    }
}
