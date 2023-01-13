use crate::{
    app::pane::{scrollable_content, style, title_bar, IterElements},
    Msg, View,
};
use iced::{
    theme,
    widget::{
        button, container, horizontal_rule, horizontal_space,
        pane_grid::{self, Content, DragEvent, Pane, ResizeEvent},
        text, text_input, toggler, Column, PaneGrid, Row,
    },
    Alignment, Element, Length, Theme,
};
use paste::paste;
use tap::Pipe;

macro_rules! edit_pane_state {
    ($($struct_name:ident: {$($field_name:ident: $field_type:ty),* $(,)?}),* $(,)?) => {
        paste! {$(

        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct [<$struct_name:camel Proxy>] {
            $(pub [<$field_name:snake>]: $field_type,)*
            pub index: usize,
        }

        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        enum [<$struct_name:camel Change>] {
            $([<$field_name:camel>]($field_type),)*
        }

        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct [<$struct_name:camel PaneChange>] {
            pub pane: Pane,
            change: [<$struct_name:camel Change>]
        }

        impl [<$struct_name:camel Proxy>] {
            fn apply_change(&mut self, change: [<$struct_name:camel Change>]) {
                match change {
                    $([<$struct_name:camel Change>] ::[<$field_name:camel>]([<$field_name:snake>]) => self. [<$field_name:snake>] = [<$field_name:snake>],)*
                }
            }
        }

        )*}
    };
}

edit_pane_state! {
    Bookmark: {
        info: String,
        url: String,
        tags: Vec<String>,
    },
    Category: {
        id: String,
        name: String,
        info: String,
        identifiers: Vec<String>,
        subcategories: Vec<String>,
    },
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PaneState {
    Settings,
    Bookmark(BookmarkProxy),
    Category(CategoryProxy),
}

#[derive(Clone, Debug)]
pub enum Message {
    BookmarkPaneChange(BookmarkPaneChange),
    CategoryPaneChange(CategoryPaneChange),
    ClosePane(Pane),
    DragPane(DragEvent),
    ResizePane(ResizeEvent),
}

#[derive(Debug)]
pub struct State {
    pub panes: pane_grid::State<PaneState>,
    pub settings_pane: Pane,
}

impl State {
    pub fn new() -> Self {
        let (panes, settings_pane) = pane_grid::State::new(PaneState::Settings);

        Self {
            panes,
            settings_pane,
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn update(&mut self, message: Message) {
        match message {
            Message::BookmarkPaneChange(pane_change) => self
                .panes
                .get_mut(&pane_change.pane)
                .expect("edited bookmark pane should exist")
                .edit_bookmark(pane_change),
            Message::CategoryPaneChange(pane_change) => self
                .panes
                .get_mut(&pane_change.pane)
                .expect("edited category pane should exist")
                .edit_category(pane_change),
            Message::ClosePane(pane) => {
                self.panes.close(&pane);
            }
            Message::DragPane(drag_event) => {
                if let DragEvent::Dropped { pane, target } = dbg!(drag_event) {
                    self.panes.swap(&pane, &target);
                }
            }
            Message::ResizePane(ResizeEvent { split, ratio }) => self.panes.resize(&split, ratio),
        };
    }

    pub fn view<'a>(&'a self, app_view: View) -> Element<'a, Msg> {
        Column::new()
            .push(
                Row::new()
                    .push(
                        button("Close All")
                            .padding(3)
                            .style(theme::Button::Destructive),
                    )
                    .push(text("Edit"))
                    .push(horizontal_space(Length::Fill))
                    .push(app_view.main_content.choice_row())
                    .padding(0)
                    .spacing(3)
                    .align_items(Alignment::Center),
            )
            .push(horizontal_rule(3))
            .push(
                PaneGrid::new(&self.panes, |pane, state, _| {
                    state.pane_content(app_view, pane)
                })
                .on_resize(10, |resize_event| {
                    Msg::EditColumnMessage(Message::ResizePane(resize_event))
                })
                .on_drag(|drag_event| Msg::EditColumnMessage(Message::DragPane(drag_event)))
                .spacing(3)
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(3)
            .spacing(3)
            .into()
    }
}

impl PaneState {
    fn settings_content<'a>(app_view: View) -> Content<'a, Msg> {
        type MsgFn = fn(String) -> Msg;
        Row::new()
            .push(
                [
                    (
                        "Info Width",
                        app_view.desc_width.1,
                        Msg::UpdateDescWidth as MsgFn,
                    ),
                    (
                        "Url Width",
                        app_view.url_width.1,
                        Msg::UpdateUrlWidth as MsgFn,
                    ),
                    (
                        "Shown",
                        app_view.shown_bookmarks.1,
                        Msg::UpdateShownBookmarks as MsgFn,
                    ),
                    ("From", app_view.shown_from.1, Msg::UpdateShownFrom as MsgFn),
                ]
                .into_iter()
                .collect_coumn(|(title, value, msg)| {
                    Row::new()
                        .push(text(title))
                        .push(
                            text_input("...", value, msg)
                                .padding(3)
                                .width(Length::Units(50)),
                        )
                        .align_items(Alignment::Center)
                        .spacing(3)
                })
                .spacing(3)
                .align_items(Alignment::End),
            )
            .push(
                [(
                    "Use Dark Theme",
                    app_view
                        .settings
                        .check("theme", &Theme::Dark)
                        .expect("theme should exist"),
                    |b: bool| Msg::SetTheme(if b { Theme::Dark } else { Theme::Light }),
                )]
                .into_iter()
                .collect_coumn(|(title, value, msg)| {
                    Row::new()
                        .push(container(text(title)).padding(3))
                        .push(toggler(None, value, msg))
                        .align_items(Alignment::Center)
                        .spacing(3)
                })
                .spacing(3)
                .align_items(Alignment::End),
            )
            .width(Length::Fill)
            .pipe(scrollable_content)
    }

    fn edit_bookmark_content<'a>(pane: Pane, bookmark: &BookmarkProxy) -> Content<'a, Msg> {
        Column::new()
            .push(
                Row::new()
                    .push(text("Info"))
                    .push(
                        text_input("...", &bookmark.info, move |value| {
                            Msg::EditColumnMessage(Message::BookmarkPaneChange(
                                BookmarkPaneChange {
                                    pane,
                                    change: BookmarkChange::Info(value),
                                },
                            ))
                        })
                        .padding(3),
                    )
                    .spacing(3)
                    .align_items(Alignment::Center),
            )
            .pipe(scrollable_content)
    }

    fn edit_category_content<'a>(_app_view: View, _category: &CategoryProxy) -> Content<'a, Msg> {
        text("edit category").pipe(scrollable_content)
    }

    pub fn edit_bookmark(&mut self, BookmarkPaneChange { pane: _, change }: BookmarkPaneChange) {
        let PaneState::Bookmark(ref mut bookmark) = self else {
            panic!("bookmark change tried on panel not representing a bookmark");
        };
        bookmark.apply_change(change);
    }

    pub fn edit_category(&mut self, CategoryPaneChange { pane: _, change }: CategoryPaneChange) {
        let PaneState::Category(ref mut category) = self else {
            panic!("category change tried on panel not representing a category");
        };
        category.apply_change(change);
    }

    pub fn pane_content<'a>(&self, app_view: View, pane: Pane) -> Content<'a, Msg> {
        match self {
            PaneState::Settings => {
                Self::settings_content(app_view).title_bar(title_bar("Settings", None))
            }

            PaneState::Bookmark(bookmark) => {
                Self::edit_bookmark_content(pane, bookmark).title_bar(title_bar(
                    "Edit Bookmark",
                    Some(Msg::EditColumnMessage(Message::ClosePane(pane))),
                ))
            }

            PaneState::Category(category) => Self::edit_category_content(app_view, category)
                .title_bar(title_bar(
                    "Edit Category",
                    Some(Msg::EditColumnMessage(Message::ClosePane(pane))),
                )),
        }
        .style(style::PANE_STYLE)
    }
}
