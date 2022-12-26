use crate::{AppView, MainContent, Msg};
use iced::{
    theme,
    widget::{
        button, column, horizontal_rule, horizontal_space, row, scrollable, text, vertical_space,
        Column,
    },
    Alignment, Element, Length,
};

fn category_row<'a>(app_view: AppView, level: &[usize]) -> Element<'a, Msg> {
    let btn = button(
        row![
            horizontal_space(Length::Units((level.len() as u16 - 1) * 24)),
            text(app_view.categories.storage[*level.last().unwrap()].name())
        ]
        .align_items(iced::Alignment::Fill)
        .spacing(0)
        .padding(0),
    )
    .on_press(Msg::ApplyCategory(level.into()))
    .style(theme::Button::Text)
    .padding(1);

    if app_view.edit_mode_active {
        row![
            button("Edit")
                .padding(1)
                .on_press(Msg::SwitchMainTo(MainContent::EditCategory)),
            btn
        ]
    } else {
        row![btn]
    }
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center)
    .into()
}

pub fn category_column<'a>(app_view: AppView) -> Element<'a, Msg> {
    let header = row![
        button("Reset")
            .on_press(Msg::Reset)
            .style(theme::Button::Destructive)
            .padding(3),
        text(format!("Categories ({}): ", app_view.category_tree.len())),
    ]
    .align_items(Alignment::Center)
    .spacing(3);

    column![
        header,
        horizontal_rule(3),
        scrollable(
            app_view
                .category_tree
                .iter()
                .fold(Column::new(), |r, l| { r.push(category_row(app_view, l)) })
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
