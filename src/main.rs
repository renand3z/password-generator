use iced::widget::{button, column, row, text};
use iced::{Alignment, Color, Element, Padding, Sandbox, Settings};

pub fn main() -> iced::Result {
    Password::run(Settings::default())
}

const PREFIX: &str = "1A!";

struct Password {
    prefix: String,
    words: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Generate,
}

impl Sandbox for Password {
    type Message = Message;

    fn new() -> Self {
        let first_password = {
            let mut words: Vec<String> = Vec::new();
            // let i = 0;

            for _ in 0..n {
                let generator = Generator::with_naming(Name::Plain).next().unwrap();
                let mut collected = generator.split("-");
                words.push(collected.next().unwrap().to_string());
                words.push(collected.next().unwrap().to_string());
            }

            return words;
        };
        let mut words = Vec::new();

        for word in &first_password {
            words.push(word.clone());
        }

        Self {
            prefix: String::from(PREFIX),
            words,
        }
    }

    fn title(&self) -> String {
        String::from("Password Generator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Generate => {
                let new_password = {
                    let mut words: Vec<String> = Vec::new();
                    // let i = 0;

                    for _ in 0..n {
                        let generator = Generator::with_naming(Name::Plain).next().unwrap();
                        let mut collected = generator.split("-");
                        words.push(collected.next().unwrap().to_string());
                        words.push(collected.next().unwrap().to_string());
                    }

                    return words;
                };
                self.words = Vec::new();

                for word in new_password {
                    self.words.push(word);
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let text_size = 33;
        let cyan = Color::from_rgb(0.0, 1.0, 1.0);
        let red = Color::from_rgb(1.0, 0.0, 0.0);

        // let mut row = Row::new();

        // row.push(text(&self.prefix));

        // for word in &self.words {
        //   row.push(text(word));
        // }

        // column![
        //   row,
        //   button("Generate").on_press(Message::Generate)
        // ]

        column![
            row![
                text(&self.prefix).size(text_size),
                text(&self.words[0]).size(text_size).style(cyan),
                text(&self.words[1]).size(text_size).style(red),
                // text(&self.words[2]).size(text_size).style(cyan),
            ],
            button("Generate").on_press(Message::Generate)
        ]
        // .spacing(20)
        .padding(Padding::from(30))
        .align_items(Alignment::Center)
        // .align_self(Alignment::Center)
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
