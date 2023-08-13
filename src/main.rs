use names::*;
use colored::*;
pub fn main () {
    let generator1 = Generator::with_naming(Name::Plain).next().unwrap();
    let generator2 = Generator::with_naming(Name::Plain).next().unwrap();

    let mut collected1 = generator1.split("-");
    let mut collected2 = generator2.split("-");

    println!(
        "from 'names': {}{}{}{}{}",
        "1A!".bold(),
        collected1.next().unwrap().to_owned().cyan(),
        collected1.next().unwrap().to_owned().yellow(),
        collected2.next().unwrap().to_owned().cyan(),
        collected2.next().unwrap().to_owned().yellow(),
    )
}