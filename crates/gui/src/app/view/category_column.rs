use std::collections::HashMap;

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
    let cat_map = app_view
        .categories
        .storage
        .iter()
        .enumerate()
        .map(|(i, c)| (<Box<str>>::from(c.id()), i))
        .collect::<HashMap<_, _>>();

    let mut cat_stack = app_view
        .infos
        .storage
        .iter()
        .flat_map(|i| {
            i.categories()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .map(|id| (0usize, <Box<str>>::from(id)))
        })
        .collect::<Vec<_>>();

    let mut cat_iter = Vec::new();
    while !cat_stack.is_empty() {
        let (level, cat_id) = cat_stack.pop().unwrap();

        if let Some(i) = cat_map.get(&cat_id) {
            let cat = &app_view.categories.storage[*i];

            cat_stack.extend(
                cat.subcategories()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|id| (level + 1, <Box<str>>::from(id))),
            );

            cat_iter.push((*i, level, cat))
        }
    }

    column![
        text("Categories: "),
        scrollable(
            cat_iter
                .into_iter()
                .fold(Column::new(), |r, (i, _l, c)| {
                    r.push(category_row(i, c))
                })
                .spacing(3)
        )
    ]
    .padding(3)
    .spacing(3)
}
