pub mod bookmarks_column;
pub mod category_column;
pub mod edit_column;
pub mod log_column;

use crate::{
    app::pane::{edit::State as EditPaneState, log::State as LogPaneState},
    MainContent, Msg, View,
};
use bookmarks_column::bookmark_column;
use category_column::category_column;
use edit_column::edit_column;
use iced::{
    theme,
    widget::{
        button, horizontal_rule, horizontal_space, pane_grid, text, text_input, toggler,
        vertical_rule, vertical_space, Column, Row,
    },
    Alignment, Element, Length,
};
use log_column::log_column;

fn tool_row<'a>(app_view: View) -> Row<'a, Msg> {
    let filter = Row::new()
        .push(text("Filter:"))
        .push(
            text_input("...", app_view.filter.1, Msg::FilterBookmarks)
                .padding(3)
                .width(300.into()),
        )
        .push(
            button("Apply")
                .on_press(Msg::ApplyFilter)
                .padding(3)
                .style(theme::Button::Positive),
        )
        .push(
            button("Reset")
                .on_press(Msg::Reset)
                .padding(3)
                .style(theme::Button::Destructive),
        )
        .padding(0)
        .spacing(3)
        .align_items(Alignment::Center);

    let edit_option = Row::new()
        .push(text("Edit"))
        .push(toggler(None, app_view.edit_mode_active, Msg::SetEditMode).width(Length::Shrink))
        .padding(0)
        .spacing(3)
        .align_items(Alignment::Center);

    Row::new()
        .push(horizontal_space(Length::Fill))
        .push(filter.width(Length::Shrink))
        .push(horizontal_space(Length::Fill))
        .push(edit_option.width(Length::Shrink))
        .align_items(iced::Alignment::Center)
        .spacing(3)
        .padding(3)
}

fn blank_column<'a>(app_view: View) -> Element<'a, Msg> {
    let header = Row::new()
        .push(
            button("Leave")
                .padding(3)
                .style(theme::Button::Destructive)
                .on_press(Msg::SwitchMainTo(MainContent::Bookmarks)),
        )
        .push(text("Blank:"))
        .push(horizontal_space(Length::Fill))
        .push(app_view.main_content.choice_row())
        .padding(0)
        .spacing(3)
        .align_items(Alignment::Center);

    Column::new()
        .push(header)
        .push(vertical_space(Length::Fill))
        .padding(3)
        .spacing(3)
        .into()
}

fn content_row<'a>(
    app_view: View,
    log_panes: &'a pane_grid::State<LogPaneState>,
    edit_panes: &'a pane_grid::State<EditPaneState>,
) -> Element<'a, Msg> {
    let main_content = match app_view.main_content {
        MainContent::Edit => edit_column(app_view, edit_panes),
        MainContent::Bookmarks => bookmark_column(app_view),
        MainContent::Log => log_column(app_view, log_panes),
        #[allow(unreachable_patterns)]
        _ => blank_column(app_view),
    };

    Row::new()
        .push(category_column(app_view))
        .push(vertical_rule(3))
        .push(main_content)
        .height(Length::Fill)
        .align_items(iced::Alignment::Start)
        .into()
}

pub fn view<'a>(
    app_view: View,
    log_panes: &'a pane_grid::State<LogPaneState>,
    edit_panes: &'a pane_grid::State<EditPaneState>,
) -> Element<'a, Msg> {
    let status = Row::new()
        .push(horizontal_space(Length::Fill))
        .push(text(app_view.status))
        .padding(3)
        .align_items(iced::Alignment::Center);

    Column::new()
        .push(tool_row(app_view))
        .push(horizontal_rule(3))
        .push(content_row(app_view, log_panes, edit_panes))
        .push(horizontal_rule(3))
        .push(status)
        .into()
}
