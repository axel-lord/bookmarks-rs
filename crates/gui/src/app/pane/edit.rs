use bookmark_library::{Bookmark, Category};
use iced::{
    widget::{
        container,
        pane_grid::{Content, Pane},
        scrollable, text, text_input, toggler, Row,
    },
    Alignment, Length, Theme,
};
use tap::Pipe;

use crate::{app::pane::IterElements, Msg, View};

use super::{style, title_bar};

#[allow(dead_code)]
#[derive(Debug)]
pub enum State {
    Settings,
    Bookmark { index: usize, bookmark: Bookmark },
    Category { index: usize, category: Category },
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
            .pipe(scrollable)
            .pipe(container)
            .padding(3)
            .pipe(Content::new)
    }

    fn edit_bookmark_content<'a>(_app_view: View) -> Content<'a, Msg> {
        todo!()
    }

    fn edit_category_content<'a>(_app_view: View) -> Content<'a, Msg> {
        todo!()
    }

    pub fn pane_content<'a>(&self, app_view: View, _pane: Pane) -> Content<'a, Msg> {
        match self {
            State::Settings => {
                Self::settings_content(app_view).title_bar(title_bar("Settings", None))
            }
            State::Bookmark {
                index: _,
                bookmark: _,
            } => Self::edit_bookmark_content(app_view),
            State::Category {
                index: _,
                category: _,
            } => Self::edit_category_content(app_view),
        }
        .style(style::PANE_STYLE)
    }
}
