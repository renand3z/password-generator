use iced::widget::{button, column, row, text};
use iced::{
    clipboard, executor, Alignment, Application, Color, Command, Element, Padding, Settings, Theme, window,
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
                let clipboard1 =
                    clipboard::write::<Message>(PREFIX.to_string() + &self.words.join(""));
        
                return clipboard1;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let text_size = 33;
        let cyan = Color::from_rgb(0.0, 1.0, 1.0);
        let red = Color::from_rgb(1.0, 0.0, 0.0);

 
        column![
            row![
                text(&self.prefix).size(text_size),
                text(&self.words[0]).size(text_size).style(cyan),
                text(&self.words[1]).size(text_size).style(red),
                text(&self.words[2]).size(text_size).style(cyan),
                text(&self.words[3]).size(text_size).style(red),
            ],
            button("Generate").on_press(Message::Generate),
            button("Copy to Clipboard").on_press(Message::CopyToClipboard),
        ]
        .spacing(20)
        .padding(Padding::from(30))
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
