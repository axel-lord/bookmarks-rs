use iced::{
    theme,
    widget::{button, horizontal_rule, horizontal_space, pane_grid, text, Column, PaneGrid, Row},
    Alignment, Element, Length,
};

use crate::{app::pane::edit::State as EditPaneState, Msg, View};

pub fn edit_column<'a>(
    app_view: View,
    edit_panes: &'a pane_grid::State<EditPaneState>,
) -> Element<'a, Msg> {
    Column::new()
        .push(
            Row::new()
                .push(
                    button("Close All")
                        .padding(3)
                        .style(theme::Button::Destructive),
                )
                .push(text("Edit"))
                .push(horizontal_space(Length::Fill))
                .push(app_view.main_content.choice_row())
                .padding(0)
                .spacing(3)
                .align_items(Alignment::Center),
        )
        .push(horizontal_rule(3))
        .push(
            PaneGrid::new(edit_panes, |pane, state, _| {
                state.pane_content(app_view, pane)
            })
            .on_resize(10, Msg::EditPaneResize)
            .on_drag(Msg::DragEditPane)
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
