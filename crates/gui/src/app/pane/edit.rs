use iced::{
    widget::{
        container,
        pane_grid::{Content, Pane},
        text, text_input, toggler, Row,
    },
    Alignment, Length, Theme,
};
use tap::Pipe;

use crate::{
    app::pane::{scrollable_content, IterElements},
    Msg, View,
};

use super::{style, title_bar};

macro_rules! edit_pane_state {
    ($($struct_name:ident: {$($field_name:ident: $field_type:ty),* $(,)?}),* $(,)?) => {
        paste! {$(

        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub struct [<$struct_name:camel Proxy>] {
            $([<$field_name:snake>]: $field_type,)*
            index: usize,
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

use paste::paste;
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
pub enum State {
    Settings,
    Bookmark(BookmarkProxy),
    Category(CategoryProxy),
}

impl State {
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
                [("Use Dark Theme", app_view.is_dark_mode, |b: bool| {
                    Msg::SetTheme(if b { Theme::Dark } else { Theme::Light })
                })]
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

    fn edit_bookmark_content<'a>(_app_view: View, _bookmark: &BookmarkProxy) -> Content<'a, Msg> {
        todo!()
    }

    fn edit_category_content<'a>(_app_view: View, _category: &CategoryProxy) -> Content<'a, Msg> {
        todo!()
    }

    pub fn edit_bookmark(&mut self, BookmarkPaneChange { pane: _, change }: BookmarkPaneChange) {
        let State::Bookmark(ref mut bookmark) = self else {
            panic!("bookmark change tried on panel not representing a bookmark");
        };
        bookmark.apply_change(change);
    }

    pub fn edit_category(&mut self, CategoryPaneChange { pane: _, change }: CategoryPaneChange) {
        let State::Category(ref mut category) = self else {
            panic!("category change tried on panel not representing a category");
        };
        category.apply_change(change);
    }

    pub fn pane_content<'a>(&self, app_view: View, _pane: Pane) -> Content<'a, Msg> {
        match self {
            State::Settings => {
                Self::settings_content(app_view).title_bar(title_bar("Settings", None))
            }
            State::Bookmark(bookmark) => Self::edit_bookmark_content(app_view, bookmark),
            State::Category(category) => Self::edit_category_content(app_view, category),
        }
        .style(style::PANE_STYLE)
    }
}
