use crate::{protocol::Message, structures::enums::game_event::GameEvent};

pub struct HandlerOutcome {
    pub message: Message,
    pub event: Option<GameEvent>,
}

impl From<Message> for HandlerOutcome {
    fn from(message: Message) -> Self {
        HandlerOutcome {
            message,
            event: None,
        }
    }
}
