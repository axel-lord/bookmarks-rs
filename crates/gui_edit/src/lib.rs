use iced::{
    widget::{
        pane_grid::{self, Pane, PaneGrid},
        text,
    },
    Element,
};
use iced_lazy::Component;

#[derive(Debug)]
pub struct EditComponent {}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Clone, Debug)]
pub enum Event {}

pub enum PaneState {
    Controls,
}

pub struct State {
    control_pane: Pane,
    panes: pane_grid::State<PaneState>,
}

impl Default for State {
    fn default() -> Self {
        let (panes, control_pane) = pane_grid::State::new(PaneState::Controls);
        Self {
            panes,
            control_pane,
        }
    }
}

fn edit_pane_grid<'a>(panes: &'a pane_grid::State<PaneState>) -> Element<'a, Event> {
    PaneGrid::new(panes, |_, _, _| pane_grid::Content::new(text("todo!"))).into()
}

impl<'a, Message> Component<Message, iced::Renderer> for EditComponent
where
    Message: From<crate::Message>,
{
    type State = State;
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        None
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event> {
        edit_pane_grid(&state.panes)
    }
}

impl EditComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EditComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<EditComponent> for Element<'a, Message>
where
    Message: 'a + From<crate::Message>,
{
    fn from(value: EditComponent) -> Self {
        iced_lazy::component(value)
    }
}
