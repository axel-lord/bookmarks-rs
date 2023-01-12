mod metric;
mod url_map;

pub mod edit;
pub mod log;

use iced::{
    theme,
    widget::{
        button, container,
        pane_grid::{Content, TitleBar},
        scrollable, text, Column, Row,
    },
    Element, Length,
};
pub use metric::{IntoMetricValue, Metric, Metrics, Value as MetricValue};
use tap::Pipe;
pub use url_map::{Tally, UrlMap};

use crate::Msg;

fn title_bar<'a>(title: impl ToString, close: Option<Msg>) -> TitleBar<'a, Msg> {
    TitleBar::new(title.pipe(text).pipe(container).padding(3))
        .controls(if let Some(msg) = close {
            Row::new().push(
                button("Close")
                    .on_press(msg)
                    .style(theme::Button::Destructive)
                    .padding(3),
            )
        } else {
            Row::new()
        })
        .always_show_controls()
        .style(theme::Container::Box)
}

fn scrollable_content<'a>(content: impl Into<Element<'a, Msg>>) -> Content<'a, Msg> {
    content
        .pipe(scrollable)
        .pipe(container)
        .padding(3)
        .width(Length::Fill)
        .height(Length::Fill)
        .pipe(Content::new)
}

trait IterElements: Iterator {
    fn collect_coumn<'a, E, F>(self, f: F) -> Column<'a, Msg>
    where
        E: Into<Element<'a, Msg>>,
        F: FnMut(Self::Item) -> E;

    fn collect_row<'a, E, F>(self, f: F) -> Row<'a, Msg>
    where
        E: Into<Element<'a, Msg>>,
        F: FnMut(Self::Item) -> E;
}

impl<I> IterElements for I
where
    I: Iterator,
{
    fn collect_row<'a, E, F>(self, mut f: F) -> Row<'a, Msg>
    where
        E: Into<Element<'a, Msg>>,
        F: FnMut(Self::Item) -> E,
    {
        self.fold(Row::new(), |row, item| row.push(f(item)))
    }

    fn collect_coumn<'a, E, F>(self, mut f: F) -> Column<'a, Msg>
    where
        E: Into<Element<'a, Msg>>,
        F: FnMut(Self::Item) -> E,
    {
        self.fold(Column::new(), |column, item| column.push(f(item)))
    }
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
