use rfd::FileDialog;
use std::fs::read_to_string;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;

use iced::widget::{button, column, container, horizontal_space, row, text, text_editor};
use iced::{application, Element, Length, Task};

#[derive(Default)]
struct MyEditor {
    content: text_editor::Content,
    error: Option<FsError>,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, FsError>),
    Open,
}

#[derive(Debug, Clone)]
enum FsError {
    DialogClosed,
    IOFailed(ErrorKind),
}

fn pick_file() -> Result<Arc<String>, FsError> {
    let path = FileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .ok_or(FsError::DialogClosed)?;

    load_file(path)
}

fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, FsError> {
    read_to_string(path)
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(FsError::IOFailed)
}

impl MyEditor {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                content: text_editor::Content::default(),
                error: None,
            },
            Task::done(Message::FileOpened(load_file(format!(
                "{}/src/main.rs",
                env!("CARGO_MANIFEST_DIR")
            )))),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        let top_bar = row![button("Open").on_press(Message::Open)];

        let text_editor = text_editor(&self.content)
            .placeholder("Start typing...")
            .on_action(Message::Edit)
            .height(Length::Fill);

        let status_bar = {
            let (line, column) = &self.content.cursor_position();

            row![
                horizontal_space(),
                text(format!("{}:{}", line + 1, column + 1))
            ]
        };

        container(column![top_bar, text_editor, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => self.content.perform(action),

            Message::FileOpened(result) => match result {
                Ok(content) => self.content = text_editor::Content::with_text(&content),
                Err(error) => self.error = Some(error),
            },

            Message::Open => match pick_file() {
                Ok(file) => self.content = text_editor::Content::with_text(&file),
                Err(error) => self.error = Some(error),
            },
        }
    }
}

pub fn main() -> iced::Result {
    application("Text Editor", MyEditor::update, MyEditor::view)
        .centered()
        .run_with(|| MyEditor::new())
}
