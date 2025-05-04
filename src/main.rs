use std::fs::read_to_string;
use std::io;
use std::path::Path;
use std::sync::Arc;

use iced::widget::{column, container, horizontal_space, row, text, text_editor};
use iced::{application, Element, Length, Task};

pub fn main() -> iced::Result {
    application("Text Editor", MyEditor::update, MyEditor::view)
        .centered()
        .run_with(|| MyEditor::new())
}

#[derive(Default)]
struct MyEditor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, io::ErrorKind>),
}

impl MyEditor {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                content: text_editor::Content::default(),
            },
            Task::done(Message::FileOpened(load_file(format!(
                "{}/src/main.rs",
                env!("CARGO_MANIFEST_DIR")
            )))),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        let (line, column) = &self.content.cursor_position();
        container(
            column![
                text_editor(&self.content)
                    .placeholder("Start typing...")
                    .on_action(Message::Edit)
                    .height(Length::Fill),
                row![
                    horizontal_space(),
                    text(format!("{}:{}", line + 1, column + 1))
                ]
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
            Message::FileOpened(result) => {
                if let Ok(file) = result {
                    self.content = text_editor::Content::with_text(&file);
                }
            }
        }
    }
}

fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    read_to_string(path)
        .map(Arc::new)
        .map_err(|error| error.kind())
}
