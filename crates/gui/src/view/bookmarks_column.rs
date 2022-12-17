use crate::Msg;
use bookmark_library::Bookmark;
use iced::{
    widget::{button, column, row, scrollable, text, Column, Row},
    Length,
};
use unicode_segmentation::UnicodeSegmentation;

fn bookmark_row<'a, Renderer>(
    index: usize,
    url_width: Option<usize>,
    desc_width: Option<usize>,
    bookmark: &Bookmark,
) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: button::StyleSheet + text::StyleSheet,
{
    row![
        button("Goto")
            .on_press(Msg::GotoBookmarkLocation(index))
            .width(Length::Shrink),
        text(
            desc_width
                .map(|w| {
                    let val = bookmark.description();
                    let letters = val
                        .grapheme_indices(true)
                        .take(w + 1)
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();

                    if letters.len() == w + 1 {
                        format!("{}...", &val[0..letters[w.saturating_sub(3)]])
                    } else {
                        val.to_string()
                    }
                })
                .unwrap_or_else(|| bookmark.description().to_string())
        )
        .width(Length::Fill),
        text(
            url_width
                .map(|w| {
                    let val = bookmark.url();
                    let letters = val
                        .grapheme_indices(true)
                        .take(w + 1)
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();

                    if letters.len() == w + 1 {
                        format!("{}...", &val[0..letters[w.saturating_sub(3)]])
                    } else {
                        val.to_string()
                    }
                })
                .unwrap_or_else(|| bookmark.url().to_string())
        )
        .width(Length::Fill),
    ]
    .spacing(3)
    .align_items(iced::Alignment::Center)
}

pub fn bookmark_column<'a, Renderer>(
    bookmarks: impl IntoIterator<Item = (usize, impl AsRef<bookmark_library::Bookmark>)>,
    bookmark_range: (usize, usize),
    url_width: Option<usize>,
    desc_width: Option<usize>,
    filter: &str,
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
                .filter(|b| filter.is_empty()
                    || b.1.as_ref().description().contains(filter)
                    || b.1.as_ref().url().contains(filter))
                .skip(bookmark_range.0)
                .take(bookmark_range.1)
                .fold(Column::new(), |r, (i, b)| {
                    r.push(bookmark_row(i, url_width, desc_width, b.as_ref()))
                })
                .spacing(3)
        )
    ]
    .padding(3)
    .spacing(3)
}
