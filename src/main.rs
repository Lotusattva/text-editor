use rfd::FileDialog;
use std::fs::read_to_string;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use iced::highlighter::{self};
use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, text, text_editor,
};
use iced::{application, Element, Font, Length, Subscription, Task};

struct MyEditor {
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<FsError>,
    theme: iced::highlighter::Theme,
    is_dirty: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), FsError>),
    Open,
    New,
    Save,
    FileSaved(Result<PathBuf, FsError>),
    ThemeSelected(highlighter::Theme),
}

#[derive(Debug, Clone)]
enum FsError {
    DialogClosed,
    IOFailed(ErrorKind),
}

fn pick_file() -> Result<(PathBuf, Arc<String>), FsError> {
    let path = FileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .ok_or(FsError::DialogClosed)?;

    load_file(path)
}

fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), FsError> {
    let content = read_to_string(&path)
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(FsError::IOFailed)?;

    Ok((path, content))
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}\\src\\main.rs", env!("CARGO_MANIFEST_DIR")))
}

// declared as async so that iced thinks this is a future
async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, FsError> {
    let path = if let Some(path) = path {
        path
    } else {
        FileDialog::new()
            .set_title("Choose a file name...")
            .save_file()
            .ok_or(FsError::DialogClosed)
            .map(|handle| handle.as_path().to_owned())?
    };

    std::fs::write(&path, text).map_err(|error| FsError::IOFailed(error.kind()))?;

    Ok(path)
}

enum Icon {
    New,
    Open,
    Save,
}

fn icon<'a>(icon: Icon) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");
    text(match icon {
        Icon::New => "\u{E800}",
        Icon::Open => "\u{F115}",
        Icon::Save => "\u{E801}",
    })
    .font(ICON_FONT)
    .into()
}

fn action<'a>(content: Element<'a, Message>, on_press: Option<Message>) -> Element<'a, Message> {
    let is_disabled = on_press.is_none();
    // Build the button & wire up the click handler if we have one
    let mut btn = button(container(content).center_x(40)).on_press_maybe(on_press);

    // Pick the built-in style function
    btn = btn.style(if is_disabled {
        iced::widget::button::secondary
    } else {
        iced::widget::button::primary
    });
    btn.into()
}

impl MyEditor {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                path: None,
                content: text_editor::Content::new(),
                error: None,
                theme: highlighter::Theme::SolarizedDark,
                is_dirty: true,
            },
            Task::done(Message::FileOpened(load_file(default_file()))),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
         keyboard::on_key_press(|key: Key, modifiers: Modifiers| {
        if key == Key::Character("S".into()) && modifiers.command() {
            Some(Message::Save)
        } else {
            None
        }
    })
    }

    fn view(&self) -> Element<'_, Message> {
        let top_bar = row![
            action(icon(Icon::New), Some(Message::New)),
            action(icon(Icon::Open), Some(Message::Open)),
            action(icon(Icon::Save), self.is_dirty.then_some(Message::Save)),
            horizontal_space(),
            pick_list(
                highlighter::Theme::ALL,
                Some(self.theme),
                Message::ThemeSelected
            ),
        ]
        .spacing(10);

        let text_editor = text_editor(&self.content)
            .placeholder("Start typing...")
            .on_action(Message::Edit)
            .height(Length::Fill)
            .highlight(
                self.path
                    .as_ref()
                    .and_then(|path| path.extension()?.to_str())
                    .unwrap_or("rs"),
                self.theme,
            );

        let status_bar = {
            let (line, column) = &self.content.cursor_position();

            let status = if let Some(FsError::IOFailed(error)) = self.error.as_ref() {
                text(error.to_string())
            } else {
                match self.path.as_deref().and_then(Path::to_str) {
                    Some(path) => text(path),
                    None => text("New file (unsaved)"),
                }
            };

            row![
                status,
                horizontal_space(),
                text(format!("{}:{}", line + 1, column + 1))
            ]
        };

        container(column![top_bar, text_editor, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.error = None;
                self.content.perform(action);
                Task::none()
            }

            Message::FileOpened(result) => match result {
                Ok((path, content)) => {
                    self.path = Some(path);
                    self.content = text_editor::Content::with_text(&content);
                    self.is_dirty = false;
                    Task::none()
                }
                Err(error) => {
                    self.error = Some(error);
                    Task::none()
                }
            },

            Message::New => {
                self.path = None;
                self.content = text_editor::Content::new();
                self.is_dirty = true;
                self.error = None;
                Task::none()
            }

            Message::Open => Task::done(Message::FileOpened(pick_file())),

            Message::Save => {
                let text = self.content.text();

                Task::perform(save_file(self.path.to_owned(), text), Message::FileSaved)
            }

            Message::FileSaved(result) => match result {
                Ok(path) => {
                    self.path = Some(path);
                    self.is_dirty = false;
                    Task::none()
                }
                Err(error) => {
                    self.error = Some(error);
                    Task::none()
                }
            },
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                Task::none()
            }
        }
    }
}

pub fn main() -> iced::Result {
    application("Text Editor", MyEditor::update, MyEditor::view)
        .subscription(MyEditor::subscription)
        .centered()
        .font(include_bytes!("../icons/editor-icons.ttf"))
        .run_with(|| MyEditor::new())
}
