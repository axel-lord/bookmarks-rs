mod metric;
mod url_map;

pub mod log;

pub use metric::{IntoMetricValue, Metric, Metrics, Value as MetricValue};
pub use url_map::{Tally, UrlMap};

pub mod style {
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
