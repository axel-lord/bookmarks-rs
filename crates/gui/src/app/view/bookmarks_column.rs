use crate::{AppView, Msg};
use bookmark_library::Bookmark;
use iced::{
    widget::{button, column, row, scrollable, text, Column, Row},
    Length,
};
use unicode_segmentation::UnicodeSegmentation;

fn bookmark_row<'a, Renderer>(
    index: usize,
    url_width: usize,
    desc_width: usize,
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
        text(if desc_width == 0 {
            let val = bookmark.description();
            let letters = val
                .grapheme_indices(true)
                .take(desc_width + 1)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            if letters.len() == desc_width + 1 {
                format!("{}...", &val[0..letters[desc_width.saturating_sub(3)]])
            } else {
                val.to_string()
            }
        } else {
            bookmark.description().to_string()
        })
        .width(Length::Fill),
        text(if url_width == 0 {
            let val = bookmark.url();
            let letters = val
                .grapheme_indices(true)
                .take(url_width + 1)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            if letters.len() == url_width + 1 {
                format!("{}...", &val[0..letters[url_width.saturating_sub(3)]])
            } else {
                val.to_string()
            }
        } else {
            bookmark.url().to_string()
        })
        .width(Length::Fill),
    ]
    .spacing(3)
    .align_items(iced::Alignment::Center)
}

pub fn bookmark_column<'a, Renderer>(app_view: AppView) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + scrollable::StyleSheet,
{
    column![
        text("Bookmarks:"),
        scrollable(
            app_view
                .bookmarks
                .iter_indexed()
                .filter(|b| app_view
                    .filter
                    .0
                    .map(|f| f.is_match(b.1.url()) || f.is_match(b.1.description()))
                    .unwrap_or(true))
                .skip(app_view.shown_from.0)
                .take(app_view.shown_bookmarks.0)
                .fold(Column::new(), |r, (i, b)| {
                    r.push(bookmark_row(
                        i,
                        app_view.url_width.0,
                        app_view.desc_width.0,
                        b,
                    ))
                })
                .spacing(3)
        )
    ]
    .padding(3)
    .spacing(3)
}
