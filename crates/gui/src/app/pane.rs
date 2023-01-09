mod metric;
mod url_map;

pub mod edit;
pub mod log;

use iced::{
    theme,
    widget::{
        button, container,
        pane_grid::{Pane, TitleBar},
        text, Row,
    },
};
pub use metric::{IntoMetricValue, Metric, Metrics, Value as MetricValue};
use tap::Pipe;
pub use url_map::{Tally, UrlMap};

use crate::Msg;

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
