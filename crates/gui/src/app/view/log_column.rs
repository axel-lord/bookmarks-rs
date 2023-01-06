use iced::{
    theme,
    widget::{button, column, horizontal_rule, horizontal_space, pane_grid, row, text, PaneGrid},
    Alignment, Element, Length,
};

use crate::{app::LogPane, Msg, View};

pub fn log_column<'a>(
    app_view: View,
    log_panes: &'a pane_grid::State<LogPane>,
) -> Element<'a, Msg> {
    let header = row![
        button("Clear").padding(3).style(theme::Button::Destructive),
        text("Status Log:"),
        horizontal_space(Length::Fill),
        app_view.main_content.choice_row(),
    ]
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center);

    let content = PaneGrid::new(log_panes, |_id, pane, _is_maximized| {
        pane.pane_content(app_view).style(theme::Container::Box)
    })
    .on_resize(10, Msg::LogPaneResize)
    .spacing(3);

    column![header, horizontal_rule(3), content,]
        .padding(3)
        .spacing(3)
        .width(Length::Fill)
        .into()
}
