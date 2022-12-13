use std::ops::Range;

use crate::Msg;
use iced::{
    widget::{button, column, row, scrollable, text, Column},
    Length,
};

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
                    r.push(
                        row![
                            button("Goto")
                                .on_press(Msg::BookmarkClicked(i))
                                .width(Length::Shrink),
                            text(b.as_ref().description()).width(Length::Fill),
                            text(b.as_ref().url()).width(Length::Fill)
                        ]
                        .align_items(iced::Alignment::Center)
                        .spacing(3),
                    )
                })
                .spacing(3)
        )
    ]
    .padding(3)
    .spacing(3)
}
