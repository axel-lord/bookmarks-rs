use std::collections::HashMap;

use crate::{AppView, Msg};
use bookmark_library::Category;
use iced::{
    theme,
    widget::{button, column, horizontal_space, row, scrollable, text, vertical_space, Column},
    Alignment, Element, Length,
};

fn category_row<'a>(index: usize, level: u16, category: &Category) -> Element<'a, Msg> {
    button(
        row![
            horizontal_space(Length::Units(level.saturating_mul(24))),
            text(category.name())
        ]
        .align_items(iced::Alignment::Fill)
        .spacing(0)
        .padding(0),
    )
    .on_press(Msg::ApplyCategory(index))
    .style(theme::Button::Text)
    .padding(0)
    .into()
}

pub fn category_column<'a>(app_view: AppView) -> Element<'a, Msg> {
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

    let header = row![
        button("Reset")
            .on_press(Msg::Reset)
            .style(theme::Button::Destructive)
            .padding(3),
        text(format!("Categories ({}): ", cat_iter.len())),
    ]
    .align_items(Alignment::Center)
    .spacing(3);

    column![
        header,
        scrollable(
            cat_iter
                .into_iter()
                .fold(Column::new(), |r, (i, l, c)| {
                    r.push(category_row(i, l, c))
                })
                .align_items(Alignment::Fill)
                .spacing(3)
        ),
        vertical_space(Length::Fill),
    ]
    .align_items(Alignment::Fill)
    .spacing(3)
    .padding(3)
    .width(Length::Shrink)
    .into()
}
