use crate::{Msg, ParsedStr, View};
use aho_corasick::AhoCorasick;
use bookmark_library::Bookmark;
use iced::{
    theme,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, scrollable, text, Column,
    },
    Color, Element, Length,
};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct BookmarkColumnState {
    pub bookmark_scrollbar_id: scrollable::Id,
    pub desc_width: ParsedStr<usize>,
    pub url_width: ParsedStr<usize>,
    pub filter: Option<AhoCorasick>,
    pub filter_str: String,
    pub shown_bookmarks: ParsedStr<usize>,
    pub shown_from: ParsedStr<usize>,
}

impl Default for BookmarkColumnState {
    fn default() -> Self {
        Self {
            bookmark_scrollbar_id: scrollable::Id::unique(),
            desc_width: 50.into(),
            filter: None,
            filter_str: String::new(),
            shown_bookmarks: 512.into(),
            shown_from: 0.into(),
            url_width: 75.into(),
        }
    }
}

fn truncated_text<'a>(content: &str, max_width: usize, theme: theme::Text) -> Element<'a, Msg> {
    if max_width == 0 {
        text(content)
    } else {
        let letters = content
            .grapheme_indices(true)
            .take(max_width + 1)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        if letters.len() == max_width + 1 {
            text(format!(
                "{}...",
                &content[0..letters[max_width.saturating_sub(3)]]
            ))
        } else {
            text(content)
        }
    }
    .width(Length::Fill)
    .style(theme)
    .into()
}

fn bookmark_row<'a>(index: usize, app_view: View, bookmark: &Bookmark) -> Element<'a, Msg> {
    let btn = button(container(
        row![
            truncated_text(
                bookmark.description(),
                app_view.desc_width.0,
                theme::Text::Default
            ),
            truncated_text(
                bookmark.url(),
                app_view.url_width.0,
                theme::Text::Color(Color::from_rgb8(64, 96, 255))
            ),
        ]
        .spacing(3)
        .align_items(iced::Alignment::Center),
    ))
    .on_press(Msg::GotoBookmarkLocation(index))
    .style(theme::Button::Text)
    .padding(1);

    if app_view.edit_mode_active {
        row![
            button("Edit").padding(1).on_press(Msg::EditBookmark(index)),
            btn
        ]
    } else {
        row![btn]
    }
    .padding(0)
    .spacing(3)
    .align_items(iced::Alignment::Center)
    .into()
}

pub fn bookmark_column<'a>(app_view: View) -> Element<'a, Msg> {
    let mut bookmark_count = 0usize;
    let mut bookmarks = app_view
        .bookmarks
        .iter_indexed()
        .filter(|b| {
            app_view.filter.0.map_or(true, |f| {
                f.is_match(b.1.url()) || f.is_match(b.1.description())
            })
        })
        .skip(app_view.shown_from.0)
        .take(app_view.shown_bookmarks.0)
        .fold(
            Vec::with_capacity(app_view.shown_bookmarks.0.saturating_mul(2)),
            |mut v, (i, b)| {
                bookmark_count += 1;
                v.push(bookmark_row(i, app_view, b));
                v.push(horizontal_rule(3).into());
                v
            },
        );

    bookmarks.pop();
    let bookmarks = scrollable(Column::with_children(bookmarks).spacing(3))
        .id(app_view.bookmark_scrollbar_id.clone());

    let header = row![
        button("Prev")
            .on_press(Msg::UpdateShownFromSteps(-1))
            .padding(3),
        button("Next")
            .on_press(Msg::UpdateShownFromSteps(1))
            .padding(3),
        text(format!(
            "Bookmarks ({}~{}/{}):",
            app_view.shown_from.0,
            bookmark_count.saturating_add(app_view.shown_from.0),
            app_view.bookmarks.storage.len(),
        )),
        horizontal_space(Length::Fill),
        app_view.main_content.choice_row(),
    ]
    .spacing(3)
    .align_items(iced::Alignment::Center);

    column![header, horizontal_rule(3), bookmarks,]
        .padding(3)
        .spacing(3)
        .width(Length::Fill)
        .into()
}