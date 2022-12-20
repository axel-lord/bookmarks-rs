use std::collections::HashMap;

use crate::{AppView, Msg};
use bookmark_library::Category;
use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text, Column, Row},
    Length,
};

fn category_row<'a, Renderer>(
    index: usize,
    level: u16,
    category: &Category,
) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: text::StyleSheet + button::StyleSheet,
{
    row![
        horizontal_space(Length::Units(level.saturating_mul(24))),
        button("Apply")
            .on_press(Msg::ApplyCategory(index))
            .padding(3),
        text(category.name()),
    ]
    .spacing(3)
    .align_items(iced::Alignment::Center)
}

pub fn category_column<'a, Renderer>(app_view: AppView) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + scrollable::StyleSheet + container::StyleSheet,
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
        .flat_map(|i| i.categories().rev().map(|id| (0u16, <Box<str>>::from(id))))
        .collect::<Vec<_>>();

    let mut cat_iter = Vec::new();
    while !cat_stack.is_empty() {
        let (level, cat_id) = cat_stack.pop().unwrap();
        if level >= 12 {
            continue;
        }

        if let Some(i) = cat_map.get(&cat_id) {
            let cat = &app_view.categories.storage[*i];

            cat_stack.extend(
                cat.subcategories()
                    .rev()
                    .map(|id| (level + 1, <Box<str>>::from(id))),
            );

            cat_iter.push((*i, level, cat))
        }
    }

    let header = row![container(text(format!("Categories ({}): ", cat_iter.len()))).padding(3),];

    column![
        header,
        scrollable(
            cat_iter
                .into_iter()
                .fold(Column::new(), |r, (i, l, c)| {
                    r.push(category_row(i, l, c))
                })
                .spacing(3)
        ),
    ]
    .padding(3)
    .spacing(3)
}
