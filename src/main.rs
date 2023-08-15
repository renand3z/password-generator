use iced::widget::{button, column, row, text};
use iced::{Alignment, Color, Element, Padding, Sandbox, Settings};
use password_generator::get_word_pair;

pub fn main() -> iced::Result {
    Password::run(Settings::default())
}

const PREFIX: &str = "1A!";
const WORDS: u8 = 2;
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
        let words = get_word_pair(WORDS);

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
                let new_words = get_word_pair(WORDS);
                    self.words = new_words;
                }
                //self.words = Vec::new();

              //  for word in new_words {
               //     self.words.push(word);
             //   }
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
