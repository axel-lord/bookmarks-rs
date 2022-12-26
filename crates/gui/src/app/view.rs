mod bookmarks_column;
mod category_column;

use crate::{AppView, MainContent, Msg};
use bookmarks_column::bookmark_column;
use category_column::category_column;
use iced::{
    theme,
    widget::{
        button, column, horizontal_rule, horizontal_space, row, scrollable, text, text_input,
        toggler, vertical_rule, vertical_space, Column,
    },
    Alignment, Element, Length,
};

fn tool_row<'a>(app_view: AppView) -> Element<'a, Msg> {
    let filter = row![
        text("Filter:"),
        text_input("...", app_view.filter.1, Msg::FilterBookmarks)
            .padding(3)
            .width(300.into()),
        button("Apply")
            .on_press(Msg::ApplyFilter)
            .padding(3)
            .style(theme::Button::Positive),
        button("Reset")
            .on_press(Msg::Reset)
            .padding(3)
            .style(theme::Button::Destructive),
    ]
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center);

    let edit_option = row![
        text("Edit"),
        toggler(None, app_view.edit_mode_active, Msg::SetEditMode).width(Length::Shrink)
    ]
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center);

    row![
        horizontal_space(Length::Fill),
        filter.width(Length::Shrink),
        horizontal_space(Length::Fill),
        edit_option.width(Length::Shrink),
    ]
    .align_items(iced::Alignment::Center)
    .spacing(3)
    .padding(3)
    .into()
}

fn settings_column<'a>(app_view: AppView) -> Element<'a, Msg> {
    let header = row![
        button("Default")
            .padding(3)
            .style(theme::Button::Destructive),
        text("Settings:"),
        horizontal_space(Length::Fill),
        app_view.main_content.choice_row(),
    ]
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center);

    let number_row = row![
        text("Info width:"),
        text_input("...", app_view.desc_width.1, Msg::UpdateDescWidth)
            .padding(3)
            .width(Length::Units(50)),
        text("URL width:"),
        text_input("...", app_view.url_width.1, Msg::UpdateUrlWidth)
            .padding(3)
            .width(Length::Units(50)),
        text("Shown:"),
        text_input("...", app_view.shown_bookmarks.1, Msg::UpdateShownBookmarks)
            .padding(3)
            .width(Length::Units(50)),
        text("From:"),
        text_input("...", app_view.shown_from.1, Msg::UpdateShownFrom)
            .padding(3)
            .width(Length::Units(50)),
        horizontal_space(Length::Fill),
    ]
    .spacing(3)
    .align_items(iced::Alignment::Center);

    column![header, number_row, vertical_space(Length::Fill),]
        .padding(3)
        .spacing(3)
        .into()
}

fn status_column<'a>(app_view: AppView) -> Element<'a, Msg> {
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

    column![header, content, vertical_space(Length::Fill),]
        .padding(3)
        .spacing(3)
        .into()
}

fn blank_column<'a>(app_view: AppView) -> Element<'a, Msg> {
    let header = row![
        button("Leave")
            .padding(3)
            .style(theme::Button::Destructive)
            .on_press(Msg::SwitchMainTo(MainContent::Bookmarks)),
        text("Blank:"),
        horizontal_space(Length::Fill),
        app_view.main_content.choice_row(),
    ]
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center);

    column![header, vertical_space(Length::Fill)]
        .padding(3)
        .spacing(3)
        .into()
}

fn content_row<'a>(app_view: AppView) -> Element<'a, Msg> {
    let main_content = match app_view.main_content {
        MainContent::Settings => settings_column(app_view),
        MainContent::Bookmarks => bookmark_column(app_view),
        MainContent::Log => status_column(app_view),

        _ => blank_column(app_view),
    };

    row![category_column(app_view), vertical_rule(3), main_content,]
        .height(Length::Fill)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn application_view<'a>(app_view: AppView<'_>) -> Element<'a, Msg> {
    let status = row![horizontal_space(Length::Fill), text(app_view.status),]
        .padding(3)
        .align_items(iced::Alignment::Center);

    column![
        tool_row(app_view),
        horizontal_rule(3),
        content_row(app_view),
        horizontal_rule(3),
        status,
    ]
    .into()
}
