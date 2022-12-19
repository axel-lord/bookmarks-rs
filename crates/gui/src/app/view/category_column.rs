use crate::{AppView, Msg};
use bookmark_library::Category;
use iced::widget::{button, column, row, scrollable, text, Column, Row};

fn category_row<'a, Renderer>(index: usize, category: &Category) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: text::StyleSheet + button::StyleSheet,
{
    row![
        button("Apply").on_press(Msg::ApplyCategory(index)),
        text(category.name()),
    ]
    .spacing(3)
    .align_items(iced::Alignment::Center)
}

pub fn category_column<'a, Renderer>(app_view: AppView) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + scrollable::StyleSheet,
{
    column![
        text("Categories: "),
        scrollable(
            app_view
                .categories
                .iter_indexed()
                .fold(Column::new(), |r, (i, c)| { r.push(category_row(i, c)) })
                .spacing(3)
        )
    ]
    .padding(3)
    .spacing(3)
}
