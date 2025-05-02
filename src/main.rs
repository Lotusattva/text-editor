use iced::widget::{button, column, container, text};
use iced::{alignment, Element, Fill};

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {
    container(
        column![
            button("Increment").on_press(Message::Increment),
            text(counter.value)
                .size(20)
                .center()
                .align_x(alignment::Horizontal::Center),
            button("Decrement").on_press(Message::Decrement),
        ]
        .spacing(10),
    )
    .padding(20)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", update, view)
}
