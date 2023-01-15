use iced::{
    executor, theme,
    widget::{button, container},
    Application, Length, Theme,
};

struct App;

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self, iced::Command::none())
    }

    fn title(&self) -> String {
        "Application".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ButtonClicked => println!("button clicked"),
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        container(
            button("Button")
                .on_press(Message::ButtonClicked)
                .style(theme::Button::Positive),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn main() -> iced::Result {
    App::run(iced::Settings::default())
}
