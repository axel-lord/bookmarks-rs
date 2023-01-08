use std::{collections::HashMap, fmt::Display, sync::mpsc, thread};

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

#[derive(Clone, Debug, Default)]
pub struct UrlMap {
    occurance_count: HashMap<String, usize>,
    parse_issues: Vec<(Bookmark, url::ParseError)>,
}

#[derive(Clone, Copy, Debug)]
pub struct UrlMapTally {
    parse_issues: usize,
    no_domain: usize,
    domains: usize,
}

impl UrlMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tally(&self) -> UrlMapTally {
        let no_domain = self.occurance_count.get("<no domain>").copied();
        let domain_count = self.occurance_count.keys().len() - usize::from(no_domain.is_some());

        UrlMapTally {
            parse_issues: self.parse_issues.len(),
            no_domain: no_domain.unwrap_or(0),
            domains: domain_count,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum LogPane {
    #[default]
    Log,
    Stats,
}

#[derive(Clone, Copy, Debug)]
pub enum Metric {
    AverageContentStringLength,
    UrlOccurances,
}

#[derive(Clone, Debug, Default)]
pub enum MetricValue {
    #[default]
    None,
    Float(f64),
    UrlMap(UrlMap),
}

#[derive(Clone, Default, Debug)]
pub struct Metrics {
    average_content_string_length: MetricValue,
    url_occurances: MetricValue,
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
            MetricValue::UrlMap(v) => write!(f, "{:?}", v.tally()),
        }
    }
}

impl Metrics {
    pub fn get(&self, metric: Metric) -> &MetricValue {
        match metric {
            Metric::AverageContentStringLength => &self.average_content_string_length,
            Metric::UrlOccurances => &self.url_occurances,
        }
    }

    pub fn set(&mut self, metric: Metric, value: MetricValue) {
        match metric {
            Metric::AverageContentStringLength => self.average_content_string_length = value,
            Metric::UrlOccurances => self.url_occurances = value,
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

    pub fn gather_url_occurances(
        tx: mpsc::Sender<ChannelMessage>,
        bookmarks: shared::BufferStorage<Bookmark>,
    ) {
        thread::spawn(move || {
            let bookmarks = bookmarks.read().expect("posioned lock");
            tx.send(ChannelMessage::GatheredMetric(Metric::UrlOccurances, {
                let mut map = UrlMap::new();

                for bookmark in bookmarks.storage.iter() {
                    let url = match url::Url::parse(bookmark.url()) {
                        Ok(url) => url,
                        Err(err) => {
                            map.parse_issues.push((bookmark.clone(), err));
                            continue;
                        }
                    };

                    let entry = map
                        .occurance_count
                        .entry(String::from(url.domain().unwrap_or("<no domain>")))
                        .or_insert(0);

                    *entry += 1;
                }

                MetricValue::UrlMap(map)
            }))
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
                        .on_press(Msg::GatherMetric(Metric::AverageContentStringLength))
                        .padding(3),
                    text("Average ContentString Length:"),
                    text(format!(
                        "[{}]",
                        app_view.metrics.average_content_string_length
                    )),
                ]
                .spacing(3)
                .align_items(Alignment::Center);

                let url_domain_count =
                    if let MetricValue::UrlMap(ref map) = app_view.metrics.url_occurances {
                        let tally = map.tally();
                        text(format!(
                            "[Count: {}, None: {}, Err: {}]",
                            tally.domains, tally.no_domain, tally.parse_issues,
                        ))
                    } else {
                        text("[None]")
                    };

                let url_domain_row = row![
                    button("Gather")
                        .on_press(Msg::GatherMetric(Metric::UrlOccurances))
                        .padding(3),
                    text("Url Domain Count:"),
                    url_domain_count,
                ]
                .spacing(3)
                .align_items(Alignment::Center);

                let content = column![content_string_length, url_domain_row]
                    .height(Length::Fill)
                    .width(Length::Shrink)
                    .align_items(Alignment::Start)
                    .spacing(3);

                let content = if let MetricValue::UrlMap(ref map) = app_view.metrics.url_occurances
                {
                    let mut occurances = map.occurance_count.iter().collect::<Vec<_>>();
                    occurances.sort_by_key(|(_, v)| *v);

                    occurances
                        .into_iter()
                        .rev()
                        .fold(content, |content, (k, v)| {
                            content.push(text(format!("{k}: {v}")))
                        })
                } else {
                    content
                };

                Content::new(
                    column![scrollable(content)]
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
