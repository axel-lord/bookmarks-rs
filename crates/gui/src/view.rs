mod bookmarks_column;
mod category_column;

use crate::{Msg, ParsedStr};
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

fn tool_row<'a, Renderer>(
    status: &str,
    shown_bookmarks: &str,
    shown_from: &str,
    url_width_str: &str,
    desc_width_str: &str,
    filter: &str,
) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + text_input::StyleSheet,
{
    row![
        button("Reset").on_press(Msg::Reset),
        text("Info width:"),
        text_input("...", desc_width_str, |s| Msg::UpdateDescWidth(s.into()))
            .width(Length::Units(50)),
        text("URL width:"),
        text_input("...", url_width_str, |s| Msg::UpdateUrlWidth(s.into()))
            .width(Length::Units(50)),
        text("Shown:"),
        text_input("...", shown_bookmarks, |s| Msg::UpdateShownBookmarks(
            s.into()
        ))
        .width(Length::Units(50)),
        text("From:"),
        text_input("...", shown_from, |s| Msg::UpdateShownFrom(s.into())).width(Length::Units(50)),
        button("Prev").on_press(Msg::UpdateShownFromSteps(-1)),
        button("Next").on_press(Msg::UpdateShownFromSteps(1)),
        text("Filter:"),
        text_input("...", filter, |s| Msg::FilterBookmarks(s.into())),
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
    bookmark_range: (usize, usize),
    url_width: Option<usize>,
    desc_width: Option<usize>,
    filter: &str,
) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        rule::StyleSheet + scrollable::StyleSheet + text::StyleSheet + button::StyleSheet,
{
    row![
        category_column(categories.iter_indexed()).width(Length::Shrink),
        vertical_rule(3),
        bookmark_column(
            bookmarks.iter_indexed(),
            bookmark_range,
            url_width,
            desc_width,
            filter,
        )
        .width(Length::Fill)
    ]
    .align_items(iced::Alignment::Start)
}

#[allow(clippy::too_many_arguments)]
pub fn application_view<'a, Renderer>(
    bookmarks: &BufferStorage<Bookmark>,
    categories: &BufferStorage<Category>,
    status: &str,
    shown_bookmarks: &str,
    shown_from: &str,
    bookmark_range: (usize, usize),
    url_width: &ParsedStr<usize>,
    desc_width: &ParsedStr<usize>,
    filter: &str,
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
        tool_row(
            status,
            shown_bookmarks,
            shown_from,
            url_width,
            desc_width,
            filter
        ),
        horizontal_rule(3),
        content_row(
            bookmarks,
            categories,
            bookmark_range,
            *url_width.value(),
            *desc_width.value(),
            filter,
        )
    ]
}
