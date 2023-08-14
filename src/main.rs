use iced::widget::{button, column, row, text};
use iced::{Alignment, Color, Element, Padding, Sandbox, Settings};
use password_generator::{format_password, get_words};

pub fn main() -> iced::Result {
    Password::run(Settings::default())
}

struct Password {
    prefix: String,
    first: String,
    second: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Generate,
}

impl Sandbox for Password {
    type Message = Message;

    fn new() -> Self {
        let first_password = get_words();
        Self {
            prefix: String::from("1A!"),
            first: String::from(&first_password[0]),
            second: String::from(&first_password[1]),
        }
    }

    fn title(&self) -> String {
        String::from("Password Generator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Generate => {
                let new_password = get_words();
                self.first = new_password[0].clone();
                self.second = new_password[1].clone();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let text_size = 33;
        let cyan = Color::from_rgb(0.0, 1.0, 1.0);
        let red = Color::from_rgb(1.0, 0.0, 0.0);

        column![
            row![
                text("1A!").size(text_size),
                text(&self.first).size(text_size).style(cyan),
                text(&self.second).size(text_size).style(red),
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
