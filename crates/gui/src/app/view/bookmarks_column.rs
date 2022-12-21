use crate::{AppView, Msg};
use bookmark_library::Bookmark;
use iced::{
    theme,
    widget::{button, column, row, scrollable, text, Column},
    Color, Element, Length,
};
use unicode_segmentation::UnicodeSegmentation;

fn bookmark_row<'a>(
    index: usize,
    url_width: usize,
    desc_width: usize,
    bookmark: &Bookmark,
) -> Element<'a, Msg> {
    button(
        row![
            text(if desc_width != 0 {
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
            text(if url_width != 0 {
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
            .style(theme::Text::Color(Color::from_rgb8(96, 64, 255)))
            .width(Length::Fill),
        ]
        .spacing(3)
        .align_items(iced::Alignment::Center),
    )
    .on_press(Msg::GotoBookmarkLocation(index))
    .style(theme::Button::Text)
    .padding(0)
    .into()
}

pub fn bookmark_column<'a>(app_view: AppView) -> Element<'a, Msg> {
    let mut bookmarks = Vec::with_capacity(app_view.shown_bookmarks.0);
    bookmarks.extend(
        app_view
            .bookmarks
            .iter_indexed()
            .filter(|b| {
                app_view
                    .filter
                    .0
                    .map(|f| f.is_match(b.1.url()) || f.is_match(b.1.description()))
                    .unwrap_or(true)
            })
            .skip(app_view.shown_from.0)
            .take(app_view.shown_bookmarks.0)
            .map(|(i, b)| bookmark_row(i, app_view.url_width.0, app_view.desc_width.0, b)),
    );

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
            bookmarks.len().saturating_add(app_view.shown_from.0),
            app_view.bookmarks.storage.len(),
        )),
    ]
    .spacing(3)
    .align_items(iced::Alignment::Center);

    column![
        header,
        scrollable(Column::with_children(bookmarks).spacing(3)),
    ]
    .padding(3)
    .spacing(3)
    .width(Length::Fill)
    .into()
}
