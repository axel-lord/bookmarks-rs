use iced::{
    theme,
    widget::{button, horizontal_rule, horizontal_space, pane_grid, text, Column, PaneGrid, Row},
    Alignment, Element, Length,
};

use crate::{app::pane::log::State as LogPaneState, Msg, View};

pub fn log_column<'a>(
    app_view: View,
    log_panes: &'a pane_grid::State<LogPaneState>,
) -> Element<'a, Msg> {
    Column::new()
        .push(
            Row::new()
                .push(button("Clear").padding(3).style(theme::Button::Destructive))
                .push(text("Log"))
                .push(horizontal_space(Length::Fill))
                .push(app_view.main_content.choice_row())
                .padding(0)
                .spacing(3)
                .align_items(Alignment::Center),
        )
        .push(horizontal_rule(3))
        .push(
            PaneGrid::new(log_panes, |pane, state, _| {
                state.pane_content(app_view, pane)
            })
            .on_resize(10, Msg::LogPaneResize)
            .on_drag(Msg::DragLogPane)
            .spacing(3)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(3)
        .spacing(3)
        .into()
}
