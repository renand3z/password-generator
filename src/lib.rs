use names::{Generator, Name};

fn get_words() -> [String; 2] {
    let generator = Generator::with_naming(Name::Plain).next().unwrap();

    let mut collected = generator.split("-");

    return [
        collected.next().unwrap().to_string(),
        collected.next().unwrap().to_string(),
    ];
}

pub fn format_password() -> String {

    let password_array: [[String; 2]; 2] = [get_words(), get_words()];
    let formatted =  format!("{}{}{}{}{}",
    "1A!",
    password_array[0][0],
    password_array[0][1],
    password_array[1][0],
    password_array[1][1]);


    return formatted;
    
}