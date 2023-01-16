use bookmark_library::{shared, Bookmark};
use conv::prelude::*;
use std::{fmt::Display, sync::mpsc, thread};

use crate::app::ChannelMessage;

use super::UrlMap;

#[derive(Clone, Copy, Debug)]
pub enum Metric {
    AverageContentStringLength,
    UrlOccurances,
}

#[derive(Clone, Debug, Default)]
pub enum Value {
    #[default]
    None,
    Float(f64),
    UrlMap(UrlMap),
}

#[derive(Clone, Default, Debug)]
pub struct Metrics {
    average_content_string_length: Value,
    url_occurances: Value,
}

pub trait IntoMetricValue {
    fn into_metric_value(self) -> Value;
}

impl Metrics {
    pub fn get(&self, metric: Metric) -> &Value {
        match metric {
            Metric::AverageContentStringLength => &self.average_content_string_length,
            Metric::UrlOccurances => &self.url_occurances,
        }
    }

    pub fn set(&mut self, metric: Metric, value: Value) {
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
            let bookmarks = bookmarks.read();
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
                .map_or_else(|| Value::None, Value::Float),
            ))
        });
    }

    pub fn gather_url_occurances(
        tx: mpsc::Sender<ChannelMessage>,
        bookmarks: shared::BufferStorage<Bookmark>,
    ) {
        thread::spawn(move || {
            let bookmarks = bookmarks.read();
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

                Value::UrlMap(map)
            }))
        });
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(f, "None"),
            Value::Float(v) => write!(f, "{v}"),
            Value::UrlMap(v) => write!(f, "{:?}", v.tally()),
        }
    }
}

impl IntoMetricValue for f64 {
    fn into_metric_value(self) -> Value {
        Value::Float(self)
    }
}

impl IntoMetricValue for () {
    fn into_metric_value(self) -> Value {
        Value::None
    }
}

impl<T> From<T> for Value
where
    T: IntoMetricValue,
{
    fn from(value: T) -> Self {
        value.into_metric_value()
    }
}

impl<T> From<Option<T>> for Value
where
    T: IntoMetricValue,
{
    fn from(value: Option<T>) -> Self {
        value.map_or_else(|| Value::None, IntoMetricValue::into_metric_value)
    }
}

impl<T, E> From<Result<T, E>> for Value
where
    T: IntoMetricValue,
{
    fn from(value: Result<T, E>) -> Self {
        value.ok().into()
    }
}
