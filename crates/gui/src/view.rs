mod bookmarks_column;
mod category_column;

use std::ops::Range;

use crate::Msg;
use bookmark_library::{container::BufferStorage, Bookmark, Category};
use bookmarks_column::bookmark_column;
use category_column::category_column;
use iced::{
    widget::{
        button, column, horizontal_rule, horizontal_space, row, rule, scrollable, text, text_input,
        vertical_rule, Column, Row,
    },
    Length,
};

fn tool_row<'a, Renderer>(status: &str, shown_bookmarks: &str) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + text_input::StyleSheet,
{
    row![
        button("Reset").on_press(Msg::Reset),
        text("Shown bookmarks:"),
        text_input("...", shown_bookmarks, |s| Msg::ShownBookmarksChanged(
            s.into()
        ))
        .width(Length::Units(50)),
        horizontal_space(Length::Fill),
        text(status),
    ]
    .align_items(iced::Alignment::Center)
    .spacing(3)
    .padding(3)
}

fn content_row<'a, Renderer>(
    bookmarks: &BufferStorage<Bookmark>,
    categories: &BufferStorage<Category>,
    bookmark_range: Range<usize>,
) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        rule::StyleSheet + scrollable::StyleSheet + text::StyleSheet + button::StyleSheet,
{
    row![
        scrollable(category_column(categories.iter_indexed()).width(Length::Shrink)),
        vertical_rule(3),
        scrollable(bookmark_column(bookmarks.iter_indexed(), bookmark_range).width(Length::Fill))
    ]
    .align_items(iced::Alignment::Start)
}

pub fn application_view<'a, Renderer>(
    bookmarks: &BufferStorage<Bookmark>,
    categories: &BufferStorage<Category>,
    status: &str,
    shown_bookmarks: &str,
    bookmark_range: Range<usize>,
) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: text::StyleSheet
        + button::StyleSheet
        + rule::StyleSheet
        + scrollable::StyleSheet
        + text_input::StyleSheet,
{
    column![
        tool_row(status, shown_bookmarks),
        horizontal_rule(3),
        content_row(bookmarks, categories, bookmark_range)
    ]
}
