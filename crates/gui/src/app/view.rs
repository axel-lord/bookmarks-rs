mod bookmarks_column;
mod category_column;

use crate::{AppView, Msg};
use bookmarks_column::bookmark_column;
use category_column::category_column;
use iced::{
    theme,
    widget::{
        button, column, horizontal_rule, horizontal_space, row, text, text_input, vertical_rule,
    },
    Element, Length,
};

fn tool_row<'a>(app_view: AppView) -> Element<'a, Msg> {
    row![
        text("Info width:"),
        text_input("...", app_view.desc_width.1, |s| Msg::UpdateDescWidth(
            s.into()
        ))
        .padding(3)
        .width(Length::Units(50)),
        text("URL width:"),
        text_input("...", app_view.url_width.1, |s| Msg::UpdateUrlWidth(
            s.into()
        ))
        .padding(3)
        .width(Length::Units(50)),
        text("Shown:"),
        text_input("...", app_view.shown_bookmarks.1, |s| {
            Msg::UpdateShownBookmarks(s.into())
        })
        .padding(3)
        .width(Length::Units(50)),
        text("From:"),
        text_input("...", app_view.shown_from.1, |s| Msg::UpdateShownFrom(
            s.into()
        ))
        .padding(3)
        .width(Length::Units(50)),
        text("Filter:"),
        text_input("...", app_view.filter.1, |s| Msg::FilterBookmarks(s.into())).padding(3),
        button("Apply")
            .on_press(Msg::ApplyFilter)
            .padding(3)
            .style(theme::Button::Positive),
        button("Reset")
            .on_press(Msg::Reset)
            .padding(3)
            .style(theme::Button::Destructive),
        horizontal_space(Length::Fill),
        text(app_view.status),
    ]
    .align_items(iced::Alignment::Center)
    .spacing(3)
    .padding(3)
    .into()
}

fn content_row<'a>(app_view: AppView) -> Element<'a, Msg> {
    row![
        category_column(app_view),
        vertical_rule(3),
        bookmark_column(app_view)
    ]
    .align_items(iced::Alignment::Start)
    .into()
}

pub fn application_view<'a>(app_view: AppView<'_>) -> Element<'a, Msg> {
    column![
        tool_row(app_view),
        horizontal_rule(3),
        content_row(app_view)
    ]
    .into()
}
