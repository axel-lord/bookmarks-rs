use iced::{
    theme,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, text, text_input,
        toggler, vertical_space,
    },
    Alignment, Element, Length, Theme,
};

use crate::{Msg, View};

pub fn settings_column<'a>(app_view: View) -> Element<'a, Msg> {
    let header = row![
        button("Default")
            .padding(3)
            .style(theme::Button::Destructive),
        text("Settings:"),
        horizontal_space(Length::Fill),
        app_view.main_content.choice_row(),
    ]
    .padding(0)
    .spacing(3)
    .align_items(Alignment::Center);

    macro_rules! text_input_column {
        ($($setting:expr, $content:expr, $message:expr),* $(,)?) => {
            column![
                $(
                    row![
                        text($setting),
                        text_input("...", $content, $message)
                            .padding(3)
                            .width(Length::Units(50)),
                    ]
                        .align_items(Alignment::Center)
                        .spacing(3),
                )*
            ]
                .spacing(3)
                .align_items(Alignment::End)
        };
    }

    macro_rules! toggler_column {
        ($($setting:expr, $content:expr, $message:expr),* $(,)?) => {
            column![
                $(
                    row![
                        container(text($setting)).padding(3),
                        toggler(None, $content, $message)
                    ]
                        .align_items(Alignment::Center)
                        .spacing(3),
                )*
            ]
                .spacing(3)
                .align_items(Alignment::End)
        };
    }

    let number_col = text_input_column![
        "Info Width:",
        app_view.desc_width.1,
        Msg::UpdateDescWidth,
        "Url Width:",
        app_view.url_width.1,
        Msg::UpdateUrlWidth,
        "Shown:",
        app_view.shown_bookmarks.1,
        Msg::UpdateShownBookmarks,
        "From:",
        app_view.shown_from.1,
        Msg::UpdateShownFrom,
    ];

    let toggler_col = toggler_column!["Use Dark Theme:", app_view.is_dark_mode, |b: bool| {
        Msg::SetTheme(if b { Theme::Dark } else { Theme::Light })
    },];

    let columns = row![
        number_col.width(Length::Shrink),
        toggler_col.width(Length::Shrink),
        horizontal_space(Length::Fill)
    ]
    .spacing(3)
    .width(Length::Fill);

    column![
        header,
        horizontal_rule(3),
        //number_row,
        columns,
        vertical_space(Length::Fill),
    ]
    .padding(3)
    .spacing(3)
    .width(Length::Fill)
    .into()
}
