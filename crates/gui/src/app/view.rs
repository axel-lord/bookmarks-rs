mod bookmarks_column;
mod category_column;

use crate::{AppView, Msg};
use bookmarks_column::bookmark_column;
use category_column::category_column;
use iced::{
    widget::{
        button, column, horizontal_rule, horizontal_space, row, rule, scrollable, text, text_input,
        vertical_rule, Column, Row,
    },
    Length,
};
use iced_native::widget::container;

fn tool_row<'a, Renderer>(app_view: AppView) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme:
        text::StyleSheet + button::StyleSheet + text_input::StyleSheet,
{
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
        button("Apply").on_press(Msg::ApplyFilter).padding(3),
        button("Reset").on_press(Msg::Reset).padding(3),
        horizontal_space(Length::Fill),
        text(app_view.status),
    ]
    .align_items(iced::Alignment::Center)
    .spacing(3)
    .padding(3)
}

fn content_row<'a, Renderer>(app_view: AppView) -> Row<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: rule::StyleSheet
        + scrollable::StyleSheet
        + text::StyleSheet
        + button::StyleSheet
        + container::StyleSheet,
{
    row![
        category_column(app_view).width(Length::Shrink),
        vertical_rule(3),
        bookmark_column(app_view).width(Length::Fill)
    ]
    .align_items(iced::Alignment::Start)
}

#[allow(clippy::too_many_arguments)]
pub fn application_view<'a, Renderer>(app_view: AppView) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: text::StyleSheet
        + button::StyleSheet
        + rule::StyleSheet
        + scrollable::StyleSheet
        + text_input::StyleSheet
        + container::StyleSheet,
{
    column![
        tool_row(app_view),
        horizontal_rule(3),
        content_row(app_view)
    ]
}
