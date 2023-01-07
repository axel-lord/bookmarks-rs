use std::{fmt::Display, sync::mpsc, thread};

use bookmark_library::{shared, Bookmark};
use conv::prelude::*;
use iced::{
    widget::{
        button, column, container, horizontal_rule,
        pane_grid::{Content, TitleBar},
        row, scrollable, text, Column,
    },
    Alignment, Length,
};

use crate::{Msg, View};

use super::ChannelMessage;

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

#[derive(Clone, Copy, Debug, Default)]
pub enum MetricValue {
    #[default]
    None,
    Float(f64),
}

#[derive(Clone, Default, Debug, Copy)]
pub struct Metrics {
    average_content_string_length: MetricValue,
}

pub trait IntoMetricValue {
    fn into_metric_value(self) -> MetricValue;
}

impl IntoMetricValue for f64 {
    fn into_metric_value(self) -> MetricValue {
        MetricValue::Float(self)
    }
}

impl IntoMetricValue for () {
    fn into_metric_value(self) -> MetricValue {
        MetricValue::None
    }
}

impl<T> From<T> for MetricValue
where
    T: IntoMetricValue,
{
    fn from(value: T) -> Self {
        value.into_metric_value()
    }
}

impl<T> From<Option<T>> for MetricValue
where
    T: IntoMetricValue,
{
    fn from(value: Option<T>) -> Self {
        value.map_or_else(|| MetricValue::None, IntoMetricValue::into_metric_value)
    }
}

impl<T, E> From<Result<T, E>> for MetricValue
where
    T: IntoMetricValue,
{
    fn from(value: Result<T, E>) -> Self {
        value.ok().into()
    }
}

impl Display for MetricValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricValue::None => write!(f, "None"),
            MetricValue::Float(v) => write!(f, "{v}"),
        }
    }
}

impl Metrics {
    pub fn get(&self, metric: Metric) -> MetricValue {
        match metric {
            Metric::AverageContentStringLength => self.average_content_string_length,
        }
    }

    pub fn set(&mut self, metric: Metric, value: MetricValue) {
        match metric {
            Metric::AverageContentStringLength => self.average_content_string_length = value,
        }
    }

    pub fn gather_average_content_string_length(
        tx: mpsc::Sender<ChannelMessage>,
        bookmarks: shared::BufferStorage<Bookmark>,
    ) {
        thread::spawn(move || {
            let bookmarks = bookmarks.read().expect("posioned lock");
            tx.send(ChannelMessage::GatheredMetric(
                Metric::AverageContentStringLength,
                (|| {
                    let sum = f64::value_from(
                        bookmarks
                            .storage
                            .iter()
                            .map(Bookmark::stored_length)
                            .sum::<usize>(),
                    )
                    .ok()?;

                    let average = sum / f64::value_from(bookmarks.storage.len()).ok()?;

                    Some(average)
                })()
                .map_or_else(|| MetricValue::None, MetricValue::Float),
            ))
        });
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
                    text(format!(
                        "[{}]",
                        app_view.metrics.average_content_string_length
                    )),
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
