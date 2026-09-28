use std::path::PathBuf;

use iced::widget::{
    button, checkbox, column, container, horizontal_space, row, scrollable, text, text_input,
    Button, Checkbox, Column, Container, Element, Row,
};
use iced::{
    executor, Alignment, Application, Color, Command, Font, Length, Settings, Size, Theme,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const APP_ID: &str = "checktab";
const ACCENT: Color = Color::from_rgb(0xd7 as f32 / 255.0, 0x99 as f32 / 255.0, 0x21 as f32 / 255.0);
const ACCENT_DIM: Color = Color::from_rgb(0x40 as f32 / 255.0, 0x35 as f32 / 255.0, 0x22 as f32 / 255.0);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    id: Uuid,
    text: String,
    done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct List {
    id: Uuid,
    name: String,
    items: Vec<Item>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Data {
    lists: Vec<List>,
}

fn data_dir() -> PathBuf {
    dirs::data_local_dir().unwrap_or_else(|| PathBuf::from(".")).join(APP_ID)
}

fn data_path() -> PathBuf {
    data_dir().join("checklists.json")
}

fn load() -> Data {
    match std::fs::read_to_string(data_path()) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| default_data()),
        Err(_) => default_data(),
    }
}

fn default_data() -> Data {
    Data {
        lists: vec![List {
            id: Uuid::new_v4(),
            name: "General".into(),
            items: vec![],
        }],
    }
}

fn save(data: &Data) {
    if let Some(dir) = data_path().parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Ok(s) = serde_json::to_string_pretty(data) {
        let _ = std::fs::write(data_path(), s);
    }
}

#[derive(Debug, Clone)]
enum Message {
    // items
    Draft(String),
    AddItem,
    ToggleItem(usize),
    RemoveItem(usize),
    ClearCompleted,
    // lists
    SelectList(usize),
    NewList,
    RenameStart(usize),
    RenameDraft(String),
    RenameCommit,
    DeleteListPress,
}

struct Checktab {
    data: Data,
    active: usize,
    draft: String,
    renaming: Option<usize>,
    rename_text: String,
    confirm_delete: bool,
}

impl Checktab {
    fn new() -> Self {
        let data = load();
        Checktab {
            data,
            active: 0,
            draft: String::new(),
            renaming: None,
            rename_text: String::new(),
            confirm_delete: false,
        }
    }

    fn active_list(&mut self) -> &mut List {
        if self.active >= self.data.lists.len() {
            self.active = 0;
        }
        &mut self.data.lists[self.active]
    }

    fn commit(&mut self) {
        save(&self.data);
    }

    fn do_update(&mut self, message: Message) {
        match message {
            Message::Draft(s) => self.draft = s,
            Message::AddItem => {
                let trimmed = self.draft.trim().to_string();
                if !trimmed.is_empty() {
                    self.active_list().items.push(Item {
                        id: Uuid::new_v4(),
                        text: trimmed,
                        done: false,
                    });
                    self.draft.clear();
                    self.commit();
                }
            }
            Message::ToggleItem(i) => {
                if let Some(item) = self.active_list().items.get_mut(i) {
                    item.done = !item.done;
                    self.commit();
                }
            }
            Message::RemoveItem(i) => {
                self.active_list().items.remove(i);
                self.commit();
            }
            Message::ClearCompleted => {
                let list = self.active_list();
                list.items.retain(|i| !i.done);
                self.commit();
            }
            Message::SelectList(i) => {
                self.active = i;
                self.renaming = None;
                self.confirm_delete = false;
            }
            Message::NewList => {
                let n = self.data.lists.len() + 1;
                self.data.lists.push(List {
                    id: Uuid::new_v4(),
                    name: format!("List {n}"),
                    items: vec![],
                });
                self.active = self.data.lists.len() - 1;
                self.commit();
            }
            Message::RenameStart(i) => {
                self.renaming = Some(i);
                self.rename_text = self.data.lists[i].name.clone();
            }
            Message::RenameDraft(s) => self.rename_text = s,
            Message::RenameCommit => {
                if let Some(i) = self.renaming {
                    let name = self.rename_text.trim();
                    if !name.is_empty() {
                        self.data.lists[i].name = name.to_string();
                    }
                    self.renaming = None;
                    self.commit();
                }
            }
            Message::DeleteListPress => {
                if !self.confirm_delete && self.data.lists.len() == 1 {
                    // keep at least one list; second press removes items instead
                    self.confirm_delete = true;
                    return;
                }
                if self.confirm_delete {
                    self.data.lists.remove(self.active);
                    if self.active >= self.data.lists.len() {
                        self.active = self.data.lists.len().saturating_sub(1);
                    }
                    self.confirm_delete = false;
                } else {
                    // first press on a multi-list setup: clear its items, ask to confirm
                    self.confirm_delete = true;
                }
                self.commit();
            }
        }
    }
}

impl Application for Checktab {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Checktab, Command<Message>) {
        (Checktab::new(), Command::none)
    }

    fn title(&self) -> String {
        format!("checktab — {}", self.active_list().name)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.do_update(message);
        Command::none
    }

    fn view(&self) -> Element<Message> {
        // ---- tab bar ----
        let mut tabs = Row::with_children(Vec::new()).spacing(4);
        for (i, list) in self.data.lists.iter().enumerate() {
            let active = i == self.active;
            let tab: Element<Message> = if self.renaming == Some(i) {
                text_input("name", &self.rename_text)
                    .on_input(Message::RenameDraft)
                    .on_submit(Message::RenameCommit)
                    .padding(4)
                    .into()
            } else {
                let label = if list.items.iter().all(|x| x.done) && !list.items.is_empty() {
                    format!("{} ✓", list.name)
                } else {
                    list.name.clone()
                };
                button(text(&label).size(14))
                    .on_press(Message::SelectList(i))
                    .on_double_click(Message::RenameStart(i))
                    .padding([6, 10])
                    .style(move |c| {
                        iced::widget::button::Style {
                            background: Some(if active {
                                ACCENT_DIM
                            } else {
                                c.background
                            }),
                            text_color: Some(if active { ACCENT } else { c.text_color }),
                            ..iced::widget::button::Style::TextButton
                        }
                    })
                    .into()
            };
            tabs = tabs.push(tab);
        }
        tabs = tabs
            .push(horizontal_space().width(Length::Fill))
            .push(
                button(if self.confirm_delete { "Sure?" } else { "+" })
                    .on_press(if self.confirm_delete {
                        Message::DeleteListPress
                    } else {
                        Message::NewList
                    })
                    .padding([4, 10])
                    .width(Length::Shrink),
            );

        // ---- new item input ----
        let input = text_input("new item", &self.draft)
            .placeholder("Add item, press Enter")
            .on_input(Message::Draft)
            .on_submit(Message::AddItem)
            .padding(8);

        // ---- items ----
        let list = &self.data.lists[self.active.min(self.data.lists.len() - 1)];
        let mut items = Column::with_children(Vec::new()).spacing(2);
        for (i, item) in list.items.iter().enumerate() {
            let check = checkbox(&item.text, item.done, move |d| Message::ToggleItem(i))
                .font(Font::DEFAULT)
                .text_shaping(iced::text::Shaping::Basic)
                .spacing(10);
            let remove = button(text("×").size(16))
                .on_press(Message::RemoveItem(i))
                .padding(2)
                .width(Length::Shrink)
                .height(Length::Shrink);
            items = items.push(
                row![check, horizontal_space().width(Length::Fill), remove]
                    .align_items(Alignment::Center),
            );
        }
        if list.items.is_empty() {
            items = items.push(text("empty").size(13));
        }

        let done = list.items.iter().filter(|i| i.done).count();
        let footer = row![
            button("Clear completed").on_press(Message::ClearCompleted).padding([4, 8]),
            horizontal_space(),
            text(format!("{done}/{} done", list.items.len())).size(13),
        ]
        .align_items(Alignment::Center)
        .spacing(8);

        // ---- layout ----
        let content = column![
            row![tabs].spacing(4).padding([8, 8, 4, 8]),
            container(input).padding([0, 8]),
            scrollable(items).height(Length::Fill),
            container(footer).padding([4, 8, 8, 8]),
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |c| iced::widget::container::Style {
                text_color: Some(c.text.color),
                background: Some(c.background.color.into()),
                ..iced::widget::container::Style::None
            })
            .into()
    }
}

fn main() -> iced::Result {
    Checktab::run(Settings {
        id: APP_ID.into(),
        window: iced::window::Settings {
            app_id: Some(APP_ID.into()),
            size: Size::new(400.0, 600.0),
            min_size: Some(Size::new(300.0, 240.0)),
            ..iced::window::Settings::default()
        },
        default_theme: Theme::Dark,
        ..Settings::default()
    })
}
