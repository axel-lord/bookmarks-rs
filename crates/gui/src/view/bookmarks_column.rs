use crate::Msg;
use iced::{
    alignment::Vertical,
    widget::{button, horizontal_space, row, rule, text, Column},
    Length,
};

pub fn bookmark_column<'a, Renderer>(
    bookmarks: impl IntoIterator<Item = (usize, impl AsRef<bookmark_library::Bookmark>)>,
) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + rule::StyleSheet,
{
    bookmarks
        .into_iter()
        .take(100)
        .fold(Column::new().push(text("Bookmarks:")), |r, (i, b)| {
            r.push(row![
                button("Goto")
                    .on_press(Msg::BookmarkClicked(i))
                    .width(Length::Shrink),
                horizontal_space(Length::Units(10)),
                text(b.as_ref().description())
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Center),
                horizontal_space(Length::Units(10)),
                text(b.as_ref().url())
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Center)
            ])
        })
}
