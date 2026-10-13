use std::path::PathBuf;

use iced::{
    widget::{
        button, checkbox, container, horizontal_space, pick_list, scrollable, text, text_input,
        Column, Row,
    },
    Alignment, Background, Border, Color, Element, Font, Length, Padding, Settings, Shadow, Size,
    Task, Theme,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const APP_ID: &str = "checktab";

const ICONS_FONT_BYTES: &[u8] = include_bytes!("../assets/icons.ttf");
const ICON_FONT: Font = Font::with_name("Symbols Nerd Font");

const ICON_PIN: &str = "\u{f08d}"; // nf-fa-thumb_tack
const ICON_EDIT: &str = "\u{f040}"; // nf-fa-pencil
const ICON_MINUS: &str = "\u{f068}"; // nf-fa-minus
const ICON_PLUS: &str = "\u{f067}"; // nf-fa-plus
const ICON_CLOSE: &str = "\u{f00d}"; // nf-fa-times
const ICON_CHECK: &str = "\u{f00c}"; // nf-fa-check

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    #[default]
    GruvboxDark,
    GruvboxLight,
    NordDark,
    NordLight,
    CatppuccinDark,
    CatppuccinLight,
    DraculaDark,
    DraculaLight,
}

impl ThemeChoice {
    pub const ALL: &'static [ThemeChoice] = &[
        ThemeChoice::GruvboxDark,
        ThemeChoice::GruvboxLight,
        ThemeChoice::NordDark,
        ThemeChoice::NordLight,
        ThemeChoice::CatppuccinDark,
        ThemeChoice::CatppuccinLight,
        ThemeChoice::DraculaDark,
        ThemeChoice::DraculaLight,
    ];

    pub fn to_iced_theme(self) -> Theme {
        match self {
            ThemeChoice::GruvboxDark => Theme::GruvboxDark,
            ThemeChoice::GruvboxLight => Theme::GruvboxLight,
            ThemeChoice::NordDark => Theme::Nord,
            ThemeChoice::NordLight => Theme::custom(
                "Nord Light".to_string(),
                iced::theme::Palette {
                    background: Color::from_rgb(236.0 / 255.0, 239.0 / 255.0, 244.0 / 255.0),
                    text: Color::from_rgb(46.0 / 255.0, 52.0 / 255.0, 64.0 / 255.0),
                    primary: Color::from_rgb(94.0 / 255.0, 129.0 / 255.0, 172.0 / 255.0),
                    success: Color::from_rgb(163.0 / 255.0, 190.0 / 255.0, 140.0 / 255.0),
                    danger: Color::from_rgb(191.0 / 255.0, 97.0 / 255.0, 106.0 / 255.0),
                },
            ),
            ThemeChoice::CatppuccinDark => Theme::CatppuccinMocha,
            ThemeChoice::CatppuccinLight => Theme::CatppuccinLatte,
            ThemeChoice::DraculaDark => Theme::Dracula,
            ThemeChoice::DraculaLight => Theme::custom(
                "Dracula Light".to_string(),
                iced::theme::Palette {
                    background: Color::from_rgb(248.0 / 255.0, 248.0 / 255.0, 242.0 / 255.0),
                    text: Color::from_rgb(40.0 / 255.0, 42.0 / 255.0, 54.0 / 255.0),
                    primary: Color::from_rgb(125.0 / 255.0, 83.0 / 255.0, 194.0 / 255.0),
                    success: Color::from_rgb(40.0 / 255.0, 160.0 / 255.0, 80.0 / 255.0),
                    danger: Color::from_rgb(210.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0),
                },
            ),
        }
    }
}

impl std::fmt::Display for ThemeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeChoice::GruvboxDark => write!(f, "Gruvbox Dark"),
            ThemeChoice::GruvboxLight => write!(f, "Gruvbox Light"),
            ThemeChoice::NordDark => write!(f, "Nord Dark"),
            ThemeChoice::NordLight => write!(f, "Nord Light"),
            ThemeChoice::CatppuccinDark => write!(f, "Catppuccin Dark"),
            ThemeChoice::CatppuccinLight => write!(f, "Catppuccin Light"),
            ThemeChoice::DraculaDark => write!(f, "Dracula Dark"),
            ThemeChoice::DraculaLight => write!(f, "Dracula Light"),
        }
    }
}

fn theme_accent(theme: ThemeChoice) -> (Color, Color) {
    match theme {
        ThemeChoice::GruvboxDark => (
            Color::from_rgb(250.0 / 255.0, 189.0 / 255.0, 47.0 / 255.0),
            Color::from_rgb(60.0 / 255.0, 56.0 / 255.0, 54.0 / 255.0),
        ),
        ThemeChoice::GruvboxLight => (
            Color::from_rgb(181.0 / 255.0, 118.0 / 255.0, 20.0 / 255.0),
            Color::from_rgb(235.0 / 255.0, 219.0 / 255.0, 178.0 / 255.0),
        ),
        ThemeChoice::NordDark => (
            Color::from_rgb(136.0 / 255.0, 192.0 / 255.0, 208.0 / 255.0),
            Color::from_rgb(46.0 / 255.0, 52.0 / 255.0, 64.0 / 255.0),
        ),
        ThemeChoice::NordLight => (
            Color::from_rgb(94.0 / 255.0, 129.0 / 255.0, 172.0 / 255.0),
            Color::from_rgb(216.0 / 255.0, 222.0 / 255.0, 233.0 / 255.0),
        ),
        ThemeChoice::CatppuccinDark => (
            Color::from_rgb(203.0 / 255.0, 166.0 / 255.0, 247.0 / 255.0),
            Color::from_rgb(49.0 / 255.0, 50.0 / 255.0, 68.0 / 255.0),
        ),
        ThemeChoice::CatppuccinLight => (
            Color::from_rgb(136.0 / 255.0, 57.0 / 255.0, 239.0 / 255.0),
            Color::from_rgb(230.0 / 255.0, 233.0 / 255.0, 239.0 / 255.0),
        ),
        ThemeChoice::DraculaDark => (
            Color::from_rgb(189.0 / 255.0, 147.0 / 255.0, 249.0 / 255.0),
            Color::from_rgb(68.0 / 255.0, 71.0 / 255.0, 90.0 / 255.0),
        ),
        ThemeChoice::DraculaLight => (
            Color::from_rgb(125.0 / 255.0, 83.0 / 255.0, 194.0 / 255.0),
            Color::from_rgb(230.0 / 255.0, 228.0 / 255.0, 237.0 / 255.0),
        ),
    }
}

#[derive(Debug, Clone)]
enum Message {
    Draft(String),
    AddItem,
    ToggleItem(usize),
    RemoveItem(usize),
    SelectList(usize),
    NewList,
    RenameStart(usize),
    RenameDraft(String),
    RenameCommit,
    DeleteListPress,
    ClearCompleted,
    ClearChecks,
    TogglePermanent,
    SelectTheme(ThemeChoice),
}

#[derive(Debug, Clone)]
struct Checktab {
    data: Data,
    active: usize,
    draft: String,
    renaming: Option<usize>,
    rename_text: String,
    confirm_delete: bool,
    storage_path: PathBuf,
}

impl Checktab {
    fn new() -> Self {
        let storage_path = Self::data_path();
        let data = load_from(&storage_path);
        Checktab {
            data,
            active: 0,
            draft: String::new(),
            renaming: None,
            rename_text: String::new(),
            confirm_delete: false,
            storage_path,
        }
    }

    fn data_path() -> PathBuf {
        let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        let p = base.join("checktab").join("checklists.json");
        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        p
    }

    fn save(&self) {
        save_to(&self.storage_path, &self.data);
    }

    fn active_list(&mut self) -> &mut List {
        self.active = self.active.min(self.data.lists.len() - 1);
        &mut self.data.lists[self.active]
    }

    fn active_name(&self) -> &str {
        self.data
            .lists
            .get(self.active.min(self.data.lists.len().saturating_sub(1)))
            .map(|l| l.name.as_str())
            .unwrap_or("")
    }

    fn title(&self) -> String {
        format!("checktab — {}", self.active_name())
    }

    fn theme(&self) -> Theme {
        self.data.theme.to_iced_theme()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        self.do_update(message);
        Task::none()
    }

    fn do_update(&mut self, message: Message) {
        match message {
            Message::Draft(s) => self.draft = s,
            Message::AddItem => {
                let t = self.draft.trim().to_string();
                if !t.is_empty() {
                    self.active_list().items.push(Item::new(t));
                    self.draft.clear();
                    self.save();
                }
            }
            Message::ToggleItem(i) => {
                if let Some(it) = self.active_list().items.get_mut(i) {
                    it.done = !it.done;
                    self.save();
                }
            }
            Message::RemoveItem(i) => {
                if self.active_list().items.len() > i {
                    self.active_list().items.remove(i);
                    self.save();
                }
            }
            Message::SelectList(i) => {
                self.active = i;
                self.confirm_delete = false;
                self.renaming = None;
            }
            Message::NewList => {
                self.confirm_delete = false;
                self.data
                    .lists
                    .push(List::new(format!("List {}", self.data.lists.len() + 1)));
                self.active = self.data.lists.len() - 1;
                self.save();
            }
            Message::RenameStart(i) => {
                self.renaming = Some(i);
                self.rename_text = self
                    .data
                    .lists
                    .get(i)
                    .map(|l| l.name.clone())
                    .unwrap_or_default();
                self.confirm_delete = false;
            }
            Message::RenameDraft(s) => self.rename_text = s,
            Message::RenameCommit => {
                if let Some(i) = self.renaming {
                    let t = self.rename_text.trim();
                    if !t.is_empty() {
                        if let Some(l) = self.data.lists.get_mut(i) {
                            l.name = t.to_string();
                        }
                    }
                }
                self.renaming = None;
                self.save();
            }
            Message::DeleteListPress => {
                let is_perm = self
                    .data
                    .lists
                    .get(self.active)
                    .map(|l| l.permanent)
                    .unwrap_or(false);
                if is_perm {
                    self.confirm_delete = false;
                    return;
                }
                if !self.confirm_delete {
                    self.confirm_delete = true;
                } else if self.data.lists.len() > 1 {
                    self.data.lists.remove(self.active);
                    self.active = self.active.min(self.data.lists.len() - 1);
                    self.confirm_delete = false;
                    self.save();
                }
            }
            Message::ClearCompleted => {
                let list = self.active_list();
                list.items.retain(|it| !it.done);
                self.save();
            }
            Message::ClearChecks => {
                let list = self.active_list();
                for item in &mut list.items {
                    item.done = false;
                }
                self.save();
            }
            Message::TogglePermanent => {
                if let Some(list) = self.data.lists.get_mut(self.active) {
                    list.permanent = !list.permanent;
                    self.confirm_delete = false;
                    self.save();
                }
            }
            Message::SelectTheme(t) => {
                self.data.theme = t;
                self.save();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let (accent, accent_dim) = theme_accent(self.data.theme);

        // ---- tab bar ----
        let mut tab_row = Row::with_children(Vec::new()).spacing(4.0);
        for (i, list) in self.data.lists.iter().enumerate() {
            let active = i == self.active;
            let tab: Element<'_, Message> = if self.renaming == Some(i) {
                text_input("name", &self.rename_text)
                    .on_input(Message::RenameDraft)
                    .on_submit(Message::RenameCommit)
                    .padding(4.0)
                    .width(Length::Fixed(120.0))
                    .into()
            } else {
                let mut tab_label = Row::new().spacing(4.0).align_y(Alignment::Center);
                if list.permanent {
                    tab_label = tab_label.push(text(ICON_PIN).font(ICON_FONT).size(12.0));
                }
                tab_label = tab_label.push(text(&list.name).size(13.0));
                if !list.items.is_empty() && list.items.iter().all(|x| x.done) {
                    tab_label = tab_label.push(text(ICON_CHECK).font(ICON_FONT).size(11.0));
                }

                let mut b = button(tab_label).padding([5.0, 8.0]);
                if active {
                    b = b.style(move |_theme, _status| button::Style {
                        background: Some(Background::Color(accent_dim)),
                        text_color: accent,
                        border: Border::default(),
                        shadow: Shadow::default(),
                    });
                }
                b.on_press(Message::SelectList(i)).into()
            };
            tab_row = tab_row.push(tab);
        }

        let active_is_permanent = self
            .data
            .lists
            .get(self.active.min(self.data.lists.len().saturating_sub(1)))
            .map(|l| l.permanent)
            .unwrap_or(false);

        let mut pin_btn = button(text(ICON_PIN).font(ICON_FONT).size(13.0))
            .on_press(Message::TogglePermanent)
            .padding([4.0, 8.0])
            .width(Length::Shrink);

        if active_is_permanent {
            pin_btn = pin_btn.style(move |_theme, _status| button::Style {
                background: Some(Background::Color(accent_dim)),
                text_color: accent,
                border: Border {
                    color: accent,
                    width: 1.0,
                    radius: 2.0.into(),
                },
                shadow: Shadow::default(),
            });
        }

        let mut del_btn = button(if self.confirm_delete {
            text("Sure?").size(13.0)
        } else {
            text(ICON_MINUS).font(ICON_FONT).size(13.0)
        })
        .padding([4.0, 8.0])
        .width(Length::Shrink);

        if !active_is_permanent {
            del_btn = del_btn.on_press(Message::DeleteListPress);
        } else {
            del_btn = del_btn.style(move |_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.03))),
                text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.25),
                border: Border::default(),
                shadow: Shadow::default(),
            });
        }

        // action buttons: pin / rename / delete / new
        let actions = Row::new()
            .spacing(4.0)
            .align_y(Alignment::Center)
            .push(pin_btn)
            .push(
                button(text(ICON_EDIT).font(ICON_FONT).size(13.0))
                    .on_press(Message::RenameStart(self.active))
                    .padding([4.0, 8.0])
                    .width(Length::Shrink),
            )
            .push(del_btn)
            .push(
                button(text(ICON_PLUS).font(ICON_FONT).size(13.0))
                    .on_press(Message::NewList)
                    .padding([4.0, 8.0])
                    .width(Length::Shrink),
            );

        let top_bar = Row::new()
            .spacing(4.0)
            .align_y(Alignment::Center)
            .push(
                scrollable(tab_row)
                    .direction(scrollable::Direction::Horizontal(
                        scrollable::Scrollbar::default(),
                    ))
                    .width(Length::Fill),
            )
            .push(actions);

        // ---- new item input ----
        let input = text_input("Add item, press Enter", &self.draft)
            .on_input(Message::Draft)
            .on_submit(Message::AddItem)
            .padding(8.0);

        // ---- items ----
        let list = &self.data.lists[self.active.min(self.data.lists.len() - 1)];
        let mut items = Column::with_children(Vec::new()).spacing(2.0);
        for (i, item) in list.items.iter().enumerate() {
            let check = checkbox(&item.text, item.done)
                .on_toggle(move |_done| Message::ToggleItem(i))
                .spacing(10.0)
                .size(14.0);
            let remove = button(text(ICON_CLOSE).font(ICON_FONT).size(12.0))
                .on_press(Message::RemoveItem(i))
                .padding(2.0)
                .width(Length::Shrink)
                .height(Length::Shrink);
            items = items.push(
                Row::new()
                    .push(check)
                    .push(horizontal_space().width(Length::Fill))
                    .push(remove)
                    .align_y(Alignment::Center),
            );
        }
        if list.items.is_empty() {
            items = items.push(text("(empty)").size(13.0));
        }

        let done = list.items.iter().filter(|i| i.done).count();
        let theme_picker = pick_list(
            ThemeChoice::ALL,
            Some(self.data.theme),
            Message::SelectTheme,
        )
        .text_size(12.0)
        .padding([3.0, 6.0]);

        let footer = Row::new()
            .push(
                button(text("Clear checks").size(13.0))
                    .on_press(Message::ClearChecks)
                    .padding([4.0, 8.0]),
            )
            .push(
                button(text("Clear completed").size(13.0))
                    .on_press(Message::ClearCompleted)
                    .padding([4.0, 8.0]),
            )
            .push(horizontal_space())
            .push(theme_picker)
            .push(text(format!("{done}/{} done", list.items.len())).size(13.0))
            .align_y(Alignment::Center)
            .spacing(6.0);

        // ---- layout ----
        let content = Column::with_children(vec![
            top_bar
                .padding(Padding {
                    top: 8.0,
                    right: 8.0,
                    bottom: 4.0,
                    left: 8.0,
                })
                .into(),
            container(input).padding([0.0, 8.0]).into(),
            scrollable(items).height(Length::Fill).into(),
            container(footer)
                .padding(Padding {
                    top: 4.0,
                    right: 8.0,
                    bottom: 8.0,
                    left: 8.0,
                })
                .into(),
        ])
        .spacing(0.0)
        .width(Length::Fill)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for Checktab {
    fn default() -> Self {
        Checktab::new()
    }
}

fn main() -> iced::Result {
    iced::application(Checktab::title, Checktab::update, Checktab::view)
        .theme(Checktab::theme)
        .font(ICONS_FONT_BYTES)
        .settings(Settings {
            id: Some(APP_ID.to_string()),
            ..Settings::default()
        })
        .window(iced::window::Settings {
            size: Size::new(400.0, 600.0),
            min_size: Some(Size::new(300.0, 240.0)),
            platform_specific: iced::window::settings::PlatformSpecific {
                application_id: APP_ID.to_string(),
                ..Default::default()
            },
            ..iced::window::Settings::default()
        })
        .run()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Data {
    #[serde(default)]
    theme: ThemeChoice,
    lists: Vec<List>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct List {
    id: String,
    name: String,
    #[serde(default)]
    permanent: bool,
    items: Vec<Item>,
}

impl List {
    fn new(name: String) -> Self {
        List {
            id: Uuid::new_v4().to_string(),
            name,
            permanent: false,
            items: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    text: String,
    done: bool,
}

impl Item {
    fn new(text: String) -> Self {
        Item { text, done: false }
    }
}

fn default_data() -> Data {
    Data {
        theme: ThemeChoice::GruvboxDark,
        lists: vec![List::new("Shopping".to_string())],
    }
}

fn load_from(p: &std::path::Path) -> Data {
    match std::fs::read_to_string(p) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| default_data()),
        Err(_) => default_data(),
    }
}

fn save_to(p: &std::path::Path, data: &Data) {
    if let Ok(s) = serde_json::to_string_pretty(data) {
        let _ = std::fs::write(p, s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_checktab(temp_file: &std::path::Path) -> Checktab {
        Checktab {
            data: Data {
                theme: ThemeChoice::GruvboxDark,
                lists: vec![
                    List {
                        id: "list-1".to_string(),
                        name: "On Boarding".to_string(),
                        permanent: true,
                        items: vec![
                            Item {
                                text: "Step 1".to_string(),
                                done: true,
                            },
                            Item {
                                text: "Step 2".to_string(),
                                done: false,
                            },
                        ],
                    },
                    List {
                        id: "list-2".to_string(),
                        name: "Scratchpad".to_string(),
                        permanent: false,
                        items: vec![
                            Item {
                                text: "Task A".to_string(),
                                done: true,
                            },
                            Item {
                                text: "Task B".to_string(),
                                done: false,
                            },
                        ],
                    },
                ],
            },
            active: 0,
            draft: String::new(),
            renaming: None,
            rename_text: String::new(),
            confirm_delete: false,
            storage_path: temp_file.to_path_buf(),
        }
    }

    #[test]
    fn test_backward_compatible_json_deserialization() {
        let json_data = r#"{
            "lists": [
                {
                    "id": "54974a9b-ff42-41e3-94c6-787e5b217e32",
                    "name": "On Boarding",
                    "items": [
                        { "text": "Hook Security Setup", "done": false }
                    ]
                }
            ]
        }"#;
        let data: Data = serde_json::from_str(json_data).expect("Should deserialize legacy json");
        assert_eq!(data.lists.len(), 1);
        assert!(!data.lists[0].permanent);
        assert_eq!(data.theme, ThemeChoice::GruvboxDark);
        assert_eq!(data.lists[0].items[0].text, "Hook Security Setup");
    }

    #[test]
    fn test_serde_with_permanent_flag() {
        let mut list = List::new("Daily Routine".to_string());
        list.permanent = true;
        let data = Data {
            theme: ThemeChoice::GruvboxDark,
            lists: vec![list],
        };
        let serialized = serde_json::to_string(&data).unwrap();
        assert!(serialized.contains(r#""permanent":true"#));

        let deserialized: Data = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.lists[0].permanent);
    }

    #[test]
    fn test_clear_checks_saves_fields() {
        let temp = std::env::temp_dir().join(format!("checktab-test-{}.json", Uuid::new_v4()));
        let mut app = create_test_checktab(&temp);

        assert_eq!(app.active_list().items.len(), 2);
        assert!(app.active_list().items[0].done);

        app.do_update(Message::ClearChecks);

        // Fields must remain intact, but all checkmarks cleared
        assert_eq!(app.active_list().items.len(), 2);
        assert_eq!(app.active_list().items[0].text, "Step 1");
        assert!(!app.active_list().items[0].done);
        assert_eq!(app.active_list().items[1].text, "Step 2");
        assert!(!app.active_list().items[1].done);

        let _ = std::fs::remove_file(temp);
    }

    #[test]
    fn test_clear_completed_removes_checked_items() {
        let temp = std::env::temp_dir().join(format!("checktab-test-{}.json", Uuid::new_v4()));
        let mut app = create_test_checktab(&temp);

        app.do_update(Message::ClearCompleted);

        // "Step 1" was done, so only "Step 2" should remain
        assert_eq!(app.active_list().items.len(), 1);
        assert_eq!(app.active_list().items[0].text, "Step 2");

        let _ = std::fs::remove_file(temp);
    }

    #[test]
    fn test_permanent_tab_protected_from_deletion() {
        let temp = std::env::temp_dir().join(format!("checktab-test-{}.json", Uuid::new_v4()));
        let mut app = create_test_checktab(&temp);
        app.active = 0; // "On Boarding", permanent: true

        app.do_update(Message::DeleteListPress);
        assert!(!app.confirm_delete);
        assert_eq!(app.data.lists.len(), 2);

        // Second press should still not delete
        app.do_update(Message::DeleteListPress);
        assert_eq!(app.data.lists.len(), 2);
        assert_eq!(app.data.lists[0].name, "On Boarding");

        let _ = std::fs::remove_file(temp);
    }

    #[test]
    fn test_toggle_permanent() {
        let temp = std::env::temp_dir().join(format!("checktab-test-{}.json", Uuid::new_v4()));
        let mut app = create_test_checktab(&temp);
        app.active = 1; // "Scratchpad", permanent: false

        assert!(!app.active_list().permanent);
        app.do_update(Message::TogglePermanent);
        assert!(app.active_list().permanent);

        // Cannot delete while permanent
        app.do_update(Message::DeleteListPress);
        assert_eq!(app.data.lists.len(), 2);

        // Toggle back to not permanent
        app.do_update(Message::TogglePermanent);
        assert!(!app.active_list().permanent);

        // Now can delete with confirmation
        app.do_update(Message::DeleteListPress);
        assert!(app.confirm_delete);
        app.do_update(Message::DeleteListPress);
        assert_eq!(app.data.lists.len(), 1);
        assert_eq!(app.data.lists[0].name, "On Boarding");

        let _ = std::fs::remove_file(temp);
    }

    #[test]
    fn test_theme_selection_all_variants() {
        let temp = std::env::temp_dir().join(format!("checktab-test-{}.json", Uuid::new_v4()));
        let mut app = create_test_checktab(&temp);

        for &theme_choice in ThemeChoice::ALL {
            app.do_update(Message::SelectTheme(theme_choice));
            assert_eq!(app.data.theme, theme_choice);
            let _iced_theme = app.theme();
            let _accent = theme_accent(theme_choice);
        }

        let _ = std::fs::remove_file(temp);
    }
}
