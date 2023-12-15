use std::{io, path::Path, sync::Arc};

use iced::{
    executor,
    widget::{column, container, horizontal_space, row, text, text_editor},
    Application, Command, Length, Settings, Theme,
};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, io::ErrorKind>),
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Editor {
                content: text_editor::Content::new(),
            },
            Command::perform(
                load_file(format!("{},src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Rio Editor")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
            }
            Message::FileOpened(result) => {
                if let Ok(content) = result {
                    self.content = text_editor::Content::with(&content)
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let input = text_editor(&self.content).on_edit(Message::Edit);

        let position = {
            let (line, col) = self.content.cursor_position();
            text(format!("{}:{}", line + 1, col + 1))
        };

        let status_bar = row![horizontal_space(Length::Fill), position];
        container(column![input, status_bar]).padding(20).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
}
