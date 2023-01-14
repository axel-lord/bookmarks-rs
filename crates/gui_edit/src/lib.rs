use iced::{
    widget::{
        button,
        pane_grid::{self, Axis, Pane, ResizeEvent},
        text, PaneGrid, Row,
    },
    Element,
};
use tap::Pipe;

#[derive(Debug, Clone, Copy)]
enum PaneContent {
    Controls,
    Blank,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ResizeEvent(ResizeEvent),
    SplitControl,
}

#[derive(Debug)]
pub struct State {
    panes: pane_grid::State<PaneContent>,
    control_pane: Pane,
}

impl State {
    pub fn new() -> Self {
        let (mut panes, control_pane) = pane_grid::State::new(PaneContent::Controls);
        panes.split(Axis::Horizontal, &control_pane, PaneContent::Blank);
        Self {
            panes,
            control_pane,
        }
    }

    fn content(&self) -> Element<Message> {
        PaneGrid::new(&self.panes, |_, state, _| {
            match state {
                PaneContent::Controls => Row::new()
                    .push(text("controls"))
                    .push(button("split").on_press(Message::SplitControl))
                    .pipe(Element::from),
                PaneContent::Blank => text("blank").pipe(Element::from),
            }
            .pipe(pane_grid::Content::new)
        })
        .on_resize(10, Message::ResizeEvent)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ResizeEvent(ResizeEvent { ref split, ratio }) => {
                self.panes.resize(split, ratio)
            }
            Message::SplitControl => {
                self.panes
                    .split(Axis::Vertical, &self.control_pane, PaneContent::Blank);
            }
        }
    }

    pub fn view<ExtMsg>(&self) -> Element<'_, ExtMsg>
    where
        ExtMsg: 'static + From<Message>,
    {
        self.content().map(ExtMsg::from)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
