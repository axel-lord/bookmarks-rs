mod bookmarks_column;
mod category_column;

use crate::Msg;
use bookmark_library::{container::BufferStorage, Bookmark, Category};
use bookmarks_column::bookmark_column;
use category_column::category_column;
use iced::{
    widget::{
        button, column, horizontal_rule, horizontal_space, row, rule, scrollable, text,
        vertical_rule, Column, Row,
    },
    Length,
};
use iced_native::alignment::Vertical;

fn tool_row<'a, Renderer>(status: &str) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: text::StyleSheet + button::StyleSheet,
{
    row![
        button("Reset").on_press(Msg::Reset),
        horizontal_space(Length::Fill),
        text(status).vertical_alignment(Vertical::Center),
    ]
}

fn content_row<'a, Renderer>(
    bookmarks: &BufferStorage<Bookmark>,
    categories: &BufferStorage<Category>,
) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        rule::StyleSheet + scrollable::StyleSheet + text::StyleSheet + button::StyleSheet,
{
    row![
        scrollable(category_column(categories.iter_indexed()).width(Length::Shrink)),
        vertical_rule(3),
        scrollable(bookmark_column(bookmarks.iter_indexed()).width(Length::Fill))
    ]
}

pub fn application_view<'a, Renderer>(
    bookmarks: &BufferStorage<Bookmark>,
    categories: &BufferStorage<Category>,
    status: &str,
) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + rule::StyleSheet + scrollable::StyleSheet,
{
    column![
        tool_row(status),
        horizontal_rule(3),
        content_row(bookmarks, categories)
    ]
}
