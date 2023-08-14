use iced::widget::{button, column, text};
use iced::{Alignment, Element, Padding, Sandbox, Settings};

pub fn main() -> iced::Result {
    Password::run(Settings::default())
}

struct Password {
    value: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Generate,
}

impl Sandbox for Password {
    type Message = Message;

    fn new() -> Self {
        Self { value: String::from("Not Clicked") }
    }

    fn title(&self) -> String {
        String::from("Password Generator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Generate => {
                self.value = "Clicked".to_string();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(&self.value).size(40),
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
