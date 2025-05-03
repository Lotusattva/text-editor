use iced::widget::{container, text_editor};
use iced::{Element, Length};

pub fn main() -> iced::Result {
    iced::run("Text editor", update, view)
}

#[derive(Default)]
struct MyEditor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

fn view(editor: &MyEditor) -> Element<'_, Message> {
    container(
        text_editor(&editor.content)
            .placeholder("Start typing...")
            .on_action(Message::Edit)
            .height(Length::Fill),
    )
    .padding(10)
    .into()
}

fn update(editor: &mut MyEditor, message: Message) {
    match message {
        Message::Edit(action) => {
            editor.content.perform(action);
        }
    }
}
