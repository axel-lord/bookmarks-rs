use std::ops::Range;

use crate::Msg;
use bookmark_library::Bookmark;
use iced::{
    widget::{button, column, row, scrollable, text, Column, Row},
    Length,
};

fn bookmark_row<'a, Renderer>(index: usize, bookmark: &Bookmark) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: button::StyleSheet + text::StyleSheet,
{
    row![
        button("Goto")
            .on_press(Msg::GotoBookmarkLocation(index))
            .width(Length::Shrink),
        text(bookmark.description()).width(Length::Fill),
        text(bookmark.url()).width(Length::Fill),
    ]
    .spacing(3)
    .align_items(iced::Alignment::Center)
}

pub fn bookmark_column<'a, Renderer>(
    bookmarks: impl IntoIterator<Item = (usize, impl AsRef<bookmark_library::Bookmark>)>,
    bookmark_range: Range<usize>,
) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + scrollable::StyleSheet,
{
    column![
        text("Bookmarks:"),
        scrollable(
            bookmarks
                .into_iter()
                .skip(bookmark_range.start)
                .take(bookmark_range.count())
                .fold(Column::new(), |r, (i, b)| {
                    r.push(bookmark_row(i, b.as_ref()))
                })
                .spacing(3)
        )
    ]
    .padding(3)
    .spacing(3)
}
