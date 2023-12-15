use iced::{
    widget::{column, container, horizontal_space, row, text, text_editor},
    Length, Sandbox, Settings, Theme,
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
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Editor {
            content: text_editor::Content::with(include_str!("main.rs")),
        }
    }

    fn title(&self) -> String {
        String::from("Rio Editor")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
            }
        }
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
