use iced::keyboard::Key;
use iced::widget::text_editor::{self, Action, Binding, Content, KeyPress};
use iced::widget::{container, row, Space};
use iced::window;
use iced::{Background, Border, Color, Element, Font, Length, Padding, Task};
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application(Editor::boot, Editor::update, Editor::view)
        .title("just-write")
        .window(window::Settings {
            fullscreen: true,
            decorations: false,
            ..Default::default()
        })
        .run()
}

struct Editor {
    content: Content,
    file_path: Option<PathBuf>,
    is_dirty: bool,
    history: Vec<String>,
    history_index: usize,
    is_fullscreen: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EditorAction(Action),

    NewFile,
    OpenFile,
    FileOpened(Option<(PathBuf, String)>),
    SaveFile,
    SaveFileAs,
    FileSaved(Option<PathBuf>),
    ToggleFullscreen,
    Undo,
    Redo,
    Quit,
}

impl Editor {
    fn boot() -> Self {
        Self {
            content: Content::new(),
            file_path: None,
            is_dirty: false,
            history: vec![String::new()],
            history_index: 0,
            is_fullscreen: true,
        }
    }

    fn push_history(&mut self) {
        let text = self.content.text();
        if self.history.last().map(|s| s.as_str()) != Some(&text) {
            self.history.truncate(self.history_index + 1);
            self.history.push(text);
            self.history_index = self.history.len() - 1;
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorAction(action) => {
                let is_edit = matches!(action, Action::Edit(_));
                self.content.perform(action);
                if is_edit {
                    self.is_dirty = true;
                    self.push_history();
                }
                Task::none()
            }
            Message::NewFile => {
                self.content = Content::new();
                self.file_path = None;
                self.is_dirty = false;
                self.history = vec![String::new()];
                self.history_index = 0;
                Task::none()
            }
            Message::OpenFile => Task::perform(
                async {
                    let handle = rfd::AsyncFileDialog::new().pick_file().await?;
                    let path = handle.path().to_path_buf();
                    let contents = std::fs::read_to_string(&path).ok()?;
                    Some((path, contents))
                },
                Message::FileOpened,
            ),
            Message::FileOpened(Some((path, text))) => {
                self.content = Content::with_text(&text);
                self.file_path = Some(path);
                self.is_dirty = false;
                self.history = vec![text];
                self.history_index = 0;
                Task::none()
            }
            Message::FileOpened(None) => Task::none(),
            Message::SaveFile => {
                if let Some(path) = self.file_path.clone() {
                    let text = self.content.text();
                    Task::perform(
                        async move {
                            std::fs::write(&path, &text).ok()?;
                            Some(path)
                        },
                        Message::FileSaved,
                    )
                } else {
                    self.update(Message::SaveFileAs)
                }
            }
            Message::SaveFileAs => {
                let text = self.content.text();
                Task::perform(
                    async move {
                        let handle = rfd::AsyncFileDialog::new().save_file().await?;
                        let path = handle.path().to_path_buf();
                        std::fs::write(&path, &text).ok()?;
                        Some(path)
                    },
                    Message::FileSaved,
                )
            }
            Message::FileSaved(Some(path)) => {
                self.file_path = Some(path);
                self.is_dirty = false;
                Task::none()
            }
            Message::FileSaved(None) => Task::none(),
            Message::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
                let mode = if self.is_fullscreen {
                    window::Mode::Fullscreen
                } else {
                    window::Mode::Windowed
                };
                window::oldest().and_then(move |id| window::set_mode(id, mode))
            }
            Message::Undo => {
                if self.history_index > 0 {
                    self.history_index -= 1;
                    let text = self.history[self.history_index].clone();
                    self.content = Content::with_text(&text);
                    self.is_dirty = true;
                }
                Task::none()
            }
            Message::Redo => {
                if self.history_index + 1 < self.history.len() {
                    self.history_index += 1;
                    let text = self.history[self.history_index].clone();
                    self.content = Content::with_text(&text);
                    self.is_dirty = true;
                }
                Task::none()
            }
            Message::Quit => iced::exit(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let bg = Color::from_rgb8(0xee, 0xee, 0xee);
        let fg = Color::from_rgb8(0x33, 0x33, 0x33);

        let editor = iced::widget::text_editor(&self.content)
            .font(Font::MONOSPACE)
            .size(18.0)
            .height(Length::Fill)
            .padding(Padding::from([20, 0]))
            .style(move |_theme, _status| text_editor::Style {
                background: Background::Color(bg),
                border: Border {
                    width: 0.0,
                    ..Default::default()
                },
                placeholder: Color::from_rgb8(0x99, 0x99, 0x99),
                value: fg,
                selection: Color::from_rgba8(0x33, 0x33, 0x33, 0.2),
            })
            .on_action(Message::EditorAction)
            .key_binding(|key_press: KeyPress| {
                let modifiers = key_press.modifiers;

                if modifiers.command() {
                    match key_press.key.as_ref() {
                        Key::Character("o") => Some(Binding::Custom(Message::OpenFile)),
                        Key::Character("s") if modifiers.shift() => {
                            Some(Binding::Custom(Message::SaveFileAs))
                        }
                        Key::Character("s") => Some(Binding::Custom(Message::SaveFile)),
                        Key::Character("n") => Some(Binding::Custom(Message::NewFile)),
                        Key::Character("f") if modifiers.shift() => {
                            Some(Binding::Custom(Message::ToggleFullscreen))
                        }
                        Key::Character("z") if modifiers.shift() => {
                            Some(Binding::Custom(Message::Redo))
                        }
                        Key::Character("z") => Some(Binding::Custom(Message::Undo)),
                        Key::Character("q") => Some(Binding::Custom(Message::Quit)),
                        _ => Binding::from_key_press(key_press),
                    }
                } else {
                    Binding::from_key_press(key_press)
                }
            });

        let layout = row![
            Space::new().width(Length::FillPortion(1)),
            container(editor)
                .width(Length::FillPortion(8))
                .height(Length::Fill),
            Space::new().width(Length::FillPortion(1)),
        ];

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(bg)),
                ..Default::default()
            })
            .into()
    }
}
