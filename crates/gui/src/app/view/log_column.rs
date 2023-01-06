use iced::{
    theme,
    widget::{
        button, column, horizontal_rule, horizontal_space, row, scrollable, text, vertical_space,
        Column,
    },
    Alignment, Element, Length,
};

use crate::{Msg, View};

pub fn log_column<'a>(app_view: View) -> Element<'a, Msg> {
    let header = row![
        button("Clear").padding(3).style(theme::Button::Destructive),
        text("Status Log:"),
        horizontal_space(Length::Fill),
        app_view.main_content.choice_row(),
    ]
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center);

    let content = scrollable(
        app_view
            .status_log
            .iter()
            .fold(Column::new(), |column, msg| column.push(text(msg))),
    );

    column![
        header,
        horizontal_rule(3),
        content,
        vertical_space(Length::Fill),
    ]
    .padding(3)
    .spacing(3)
    .width(Length::Fill)
    .into()
}
