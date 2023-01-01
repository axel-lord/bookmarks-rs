use iced::{
    theme,
    widget::{
        button, column, horizontal_rule, horizontal_space, row, text, text_input, vertical_space,
    },
    Alignment, Element, Length,
};

use crate::{AppView, Msg};

pub fn settings_column<'a>(app_view: AppView) -> Element<'a, Msg> {
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

    macro_rules! setting_col {
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

    // let number_row = row![
    //     text("Info width:"),
    //     text_input("...", app_view.desc_width.1, Msg::UpdateDescWidth)
    //         .padding(3)
    //         .width(Length::Units(50)),
    //     text("URL width:"),
    //     text_input("...", app_view.url_width.1, Msg::UpdateUrlWidth)
    //         .padding(3)
    //         .width(Length::Units(50)),
    //     text("Shown:"),
    //     text_input("...", app_view.shown_bookmarks.1, Msg::UpdateShownBookmarks)
    //         .padding(3)
    //         .width(Length::Units(50)),
    //     text("From:"),
    //     text_input("...", app_view.shown_from.1, Msg::UpdateShownFrom)
    //         .padding(3)
    //         .width(Length::Units(50)),
    //     horizontal_space(Length::Fill),
    // ]
    // .spacing(3)
    // .align_items(iced::Alignment::Center);

    let number_col = setting_col![
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

    let columns = row![
        number_col.width(Length::Shrink),
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
