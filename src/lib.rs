use names::{Generator, Name};

pub fn get_words() -> [String; 2] {
    let generator = Generator::with_naming(Name::Plain).next().unwrap();

    let mut collected = generator.split("-");

    return [
        collected.next().unwrap().to_string(),
        collected.next().unwrap().to_string(),
    ];
}
