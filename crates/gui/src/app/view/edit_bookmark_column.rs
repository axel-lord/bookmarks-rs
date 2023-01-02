use iced::{
    theme,
    widget::{button, column, horizontal_rule, horizontal_space, row, text, vertical_space},
    Alignment, Element, Length,
};

use crate::{AppView, MainContent, Msg};

#[allow(dead_code)]
pub fn edit_bookmark_column<'a>(_app_view: AppView) -> Element<'a, Msg> {
    let header = row![
        button("Ok").padding(3).style(theme::Button::Positive),
        button("Cancel")
            .padding(3)
            .style(theme::Button::Destructive)
            .on_press(Msg::SwitchMainTo(MainContent::Bookmarks)),
        text("Edit Bookmark:"),
        horizontal_space(Length::Fill),
    ]
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center);

    column![header, horizontal_rule(3), vertical_space(Length::Fill)]
        .padding(3)
        .spacing(3)
        .width(Length::Fill)
        .into()
}
