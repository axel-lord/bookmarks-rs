mod metric;
mod url_map;
pub use metric::{IntoMetricValue, Metric, Metrics, Value as MetricValue};
use tap::{Pipe, Tap};
pub use url_map::{Tally, UrlMap};

use iced::{
    theme,
    widget::{
        button, container,
        pane_grid::{Content, Pane, TitleBar},
        scrollable, text, Column, Row,
    },
    Alignment, Length,
};

use crate::{Msg, View};

#[derive(Clone, Default, Debug)]
pub enum LogPane {
    #[default]
    Log,
    Stats,
    UrlSummary(UrlMap),
}

impl From<UrlMap> for LogPane {
    fn from(v: UrlMap) -> Self {
        Self::UrlSummary(v)
    }
}

impl LogPane {
    fn title_bar<'a>(title: impl ToString, close: Option<Pane>) -> TitleBar<'a, Msg> {
        TitleBar::new(title.pipe(text).pipe(container).padding(3))
            .controls(if let Some(pane) = close {
                Row::new().push(
                    button("Close")
                        .on_press(Msg::CloseLogPane(pane))
                        .style(theme::Button::Destructive)
                        .padding(3),
                )
            } else {
                Row::new()
            })
            .always_show_controls()
            .style(theme::Container::Box)
    }

    fn log_content<'a>(app_view: View) -> Content<'a, Msg> {
        app_view
            .status_log
            .iter()
            .fold(Column::new(), |column, msg| column.push(text(msg)))
            .width(Length::Fill)
            .pipe(scrollable)
            .pipe(container)
            .padding(3)
            .width(Length::Fill)
            .height(Length::Fill)
            .pipe(Content::new)
    }

    fn stat_content<'a>(app_view: View) -> Content<'a, Msg> {
        [Metric::AverageContentStringLength, Metric::UrlOccurances]
            .into_iter()
            .fold(Column::new(), |column, metric| {
                column.push(
                    Row::new()
                        .push(
                            button("Gather")
                                .on_press(Msg::GatherMetric(metric))
                                .padding(3),
                        )
                        .push(text(format!(
                            "{metric:?} [{}]",
                            app_view.metrics.get(metric)
                        )))
                        .spacing(3)
                        .align_items(Alignment::Center)
                        .width(Length::Fill),
                )
            })
            .align_items(Alignment::Start)
            .spacing(3)
            .pipe(scrollable)
            .pipe(container)
            .padding(3)
            .width(Length::Fill)
            .height(Length::Fill)
            .pipe(Content::new)
    }

    fn url_summary_content<'a>(url_map: &UrlMap) -> Content<'a, Msg> {
        url_map
            .occurance_count
            .iter()
            .collect::<Vec<_>>()
            .tap_mut(|vec| vec.sort_by_key(|(_, count)| *count))
            .into_iter()
            .rev()
            .fold(Column::new(), |column, (url, count)| {
                column.push(text(format!("{url}: {count}")).width(Length::Fill))
            })
            .align_items(Alignment::Start)
            .spacing(3)
            .pipe(scrollable)
            .pipe(container)
            .padding(3)
            .height(Length::Fill)
            .width(Length::Fill)
            .pipe(Content::new)
    }

    pub fn pane_content<'a>(&self, app_view: View, pane: Pane) -> Content<'a, Msg> {
        match self {
            LogPane::Log => {
                Self::log_content(app_view).title_bar(Self::title_bar("Status Log", None))
            }
            LogPane::Stats => {
                Self::stat_content(app_view).title_bar(Self::title_bar("Statistics", None))
            }
            LogPane::UrlSummary(url_map) => Self::url_summary_content(url_map)
                .title_bar(Self::title_bar("URL Summary", Some(pane))),
        }
        .style(style::PANE_STYLE)
    }
}

mod style {
    use iced::{widget::container, Theme};

    fn pane_style(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(palette.background.base.color.into()),
            text_color: Some(palette.background.base.text),
            ..Default::default()
        }
    }

    pub const PANE_STYLE: fn(&Theme) -> container::Appearance = pane_style;
}
