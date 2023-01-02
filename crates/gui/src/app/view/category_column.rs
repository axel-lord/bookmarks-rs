use crate::{AppView, Msg};
use iced::{
    theme,
    widget::{
        button, column, horizontal_rule, horizontal_space, row, scrollable, text, Column, Row,
    },
    Alignment, Element, Length,
};

fn category_row<'a>(app_view: AppView, level: &[usize], style: theme::Button) -> Element<'a, Msg> {
    let index = match level.last().cloned() {
        Some(index) => index,
        None => panic!("level param is empty"),
    };
    let category = &app_view.categories.storage[index];
    let indent_width = level.len() * 24 - 24;

    let mut row_content = Vec::<Element<'a, Msg>>::with_capacity(4);

    if app_view.edit_mode_active {
        row_content.extend([
            button("Edit")
                .padding(1)
                .on_press(Msg::EditCategory(index))
                .into(),
            horizontal_space(Length::Units(3)).into(),
        ]);
    }

    row_content.extend([
        horizontal_space(Length::Units(indent_width as u16)).into(),
        button(column![text(category.name())].align_items(Alignment::Center))
            .on_press(Msg::ApplyCategory(level.into()))
            .style(style)
            .padding(1)
            .width(Length::Units(150))
            .into(),
    ]);

    Row::with_children(row_content)
        .padding(0)
        .spacing(0)
        .align_items(Alignment::Center)
        .width(Length::Shrink)
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
                .zip(
                    [
                        || theme::Button::Positive,
                        || theme::Button::Destructive,
                        || theme::Button::Primary,
                    ]
                    .into_iter()
                    .cycle()
                    .map(|f| f())
                )
                .fold(Column::new(), |r, (l, s)| {
                    r.push(category_row(app_view, l, s))
                })
                .align_items(Alignment::Fill)
                .spacing(3)
                .width(Length::Shrink)
        ),
    ]
    .align_items(Alignment::Fill)
    .spacing(3)
    .padding(3)
    .width(Length::Shrink)
    .into()
}
