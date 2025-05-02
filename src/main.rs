use iced::widget::{button, column, text};
use iced::Element;

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {
    column![
        text(counter.value).size(20),
        button("Increment").on_press(Message::Increment),
    ]
    .spacing(10)
    .into()
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", update, view)
}
