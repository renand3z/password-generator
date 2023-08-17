use iced::widget::{button, column, row, text};
use iced::{
    clipboard, executor, Alignment, Application, Color, Command, Element, Padding, Settings, Theme,
};
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
    CopyToClipboard,
}

impl Application for Password {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Password, Command<Self::Message>) {
        let words = get_word_pair(WORDS);
        // clipboard::write::<Message>("lol it's coppying".to_string());

        (
            Self {
                prefix: String::from(PREFIX),
                words,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("1A! Password Generator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Generate => {
                let new_words = get_word_pair(WORDS);
                self.words = new_words;
                return Command::none();
            }
            Message::CopyToClipboard => {
                let clipboard1 = clipboard::write::<Message>(PREFIX.to_string() + &self.words.join(""));
                // let clipboard = clipboard::write::<Message>("lol".to_owned());
                return clipboard1;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let text_size = 33;
        let cyan = Color::from_rgb(0.0, 1.0, 1.0);
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        // let clipboard1 = clipboard::write::<Message>("lol".to_string());

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
            button("Generate").on_press(Message::Generate),
                    button("Copy to Clipboard")
        .on_press(Message::CopyToClipboard)
        ],
        ]
        // button.on_press(Message::Generate)],
        // text (Message::generic(Message::Generate)),
        // .button("Copy to Clipboard")
        // .on_press(Message::CopyToClipboard)
        // // .spacing(20)
        .padding(Padding::from(30))
        .align_items(Alignment::Center)
        // .align_self(Alignment::Center)
        .into()
        
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
