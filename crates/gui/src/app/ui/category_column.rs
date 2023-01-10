use crate::{Msg, View};
use iced::{
    theme,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, scrollable, text,
        Column, Row,
    },
    Alignment, Element, Length,
};

fn category_row<'a>(app_view: View, level: &[usize]) -> Element<'a, Msg> {
    const BUTTON_THEMES: &[&dyn Fn() -> theme::Button] = &[
        &|| theme::Button::Destructive,
        &|| theme::Button::Primary,
        &|| theme::Button::Positive,
    ];
    let Some(index) = level.last().copied() else {
        panic!("level param is empty");
    };
    let category = &app_view.categories.storage[index];
    let indent_width = level.len() * 24 - 24;

    let mut row_content = Vec::<Element<'a, Msg>>::with_capacity(4);

    if app_view.edit_mode_active {
        row_content.extend([
            button("Edit")
                .padding(3)
                .on_press(Msg::EditCategory(index))
                .into(),
            horizontal_space(Length::Units(3)).into(),
        ]);
    }

    row_content.extend([
        horizontal_space(Length::Units(u16::try_from(indent_width).expect(
            "depth of nested categories times 24 should not exceed u16::MAX",
        )))
        .into(),
        button(
            container(
                column![text(category.name())]
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .style(style::CATEGORY_INNER)
            .padding(1),
        )
        .on_press(Msg::ApplyCategory(level.into()))
        .style(BUTTON_THEMES[level.len() % BUTTON_THEMES.len()]())
        .padding(2)
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

pub fn category_column<'a>(app_view: View) -> Element<'a, Msg> {
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
                .width(Length::Shrink)
        ),
    ]
    .align_items(Alignment::Fill)
    .spacing(3)
    .padding(3)
    .width(Length::Shrink)
    .into()
}

mod style {
    use iced::{widget::container, Theme};

    fn category_inner_impl(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            background: Some(palette.background.base.color.into()),
            text_color: Some(palette.background.base.text),
            ..Default::default()
        }
    }

    pub const CATEGORY_INNER: fn(&Theme) -> container::Appearance = category_inner_impl;
}
