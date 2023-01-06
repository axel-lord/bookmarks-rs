use iced::{
    widget::{
        button, column, container, horizontal_rule,
        pane_grid::{Content, TitleBar},
        row, scrollable, text, Column,
    },
    Alignment, Length,
};

use crate::{Msg, View};

#[derive(Clone, Copy, Default, Debug)]
pub enum LogPane {
    #[default]
    Log,
    Stats,
}

#[derive(Clone, Copy, Debug)]
pub enum Metric {
    AverageContentStringLength,
}

#[derive(Clone, Default, Debug, Copy)]
pub struct Metrics {
    average_content_string_length: Option<f64>,
}

impl Metrics {
    pub fn get(&self, metric: Metric) -> Option<f64> {
        match metric {
            Metric::AverageContentStringLength => self.average_content_string_length,
        }
    }

    pub fn set(&mut self, metric: Metric, value: Option<f64>) {
        match metric {
            Metric::AverageContentStringLength => self.average_content_string_length = value,
        }
    }
}

impl LogPane {
    pub fn pane_content<'a>(self, app_view: View) -> Content<'a, Msg> {
        match self {
            LogPane::Log => {
                let content = scrollable(
                    app_view
                        .status_log
                        .iter()
                        .fold(Column::new(), |column, msg| column.push(text(msg))),
                );
                Content::new(
                    container(content)
                        .padding(3)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .title_bar(
                    TitleBar::new(column![text("Status Messages"), horizontal_rule(3)].spacing(3))
                        .padding(3),
                )
            }
            LogPane::Stats => {
                let content_string_length = row![
                    button("Gather")
                        .on_press(Msg::GatherMetric(Metric::AverageContentStringLength)),
                    text("Average ContentString Length:"),
                    app_view
                        .metrics
                        .average_content_string_length
                        .map_or_else(|| text("[None]"), |v| text(format!("[{v}]"))),
                ]
                .spacing(3)
                .align_items(Alignment::Center);

                let content = column![content_string_length]
                    .height(Length::Fill)
                    .width(Length::Shrink)
                    .align_items(Alignment::Start);

                Content::new(
                    column![content]
                        .spacing(3)
                        .padding(3)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .title_bar(
                    TitleBar::new(column![text("Stats"), horizontal_rule(3)].spacing(3)).padding(3),
                )
            }
        }
    }
}
