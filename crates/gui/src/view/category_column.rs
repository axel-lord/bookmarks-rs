use crate::Msg;
use iced::widget::{button, row, text, Column};

pub fn category_column<'a, Renderer>(
    categories: impl IntoIterator<Item = (usize, impl AsRef<bookmark_library::Category>)>,
) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: text::StyleSheet + button::StyleSheet,
{
    categories
        .into_iter()
        .fold(Column::new().push(text("Categories:")), |r, (i, c)| {
            r.push(
                row![
                    button("Apply").on_press(Msg::CategoryClicked(i)),
                    text(c.as_ref().name().to_string()),
                ]
                .spacing(3)
                .align_items(iced::Alignment::Center),
            )
        })
        .padding(3)
        .spacing(3)
}
