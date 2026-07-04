use tracing::info;

use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::enums::error::ErrorCode,
};

#[cfg(test)]
mod tests;

pub(super) fn item_request(server_info: &SharedServer, args: &Vec<String>) -> Message {
    let binding = server_info.lock().unwrap();

    let item_ref = args.join(" ");

    let item = match binding.resolve_item(&item_ref) {
        Some(item) => item,
        None => {
            return Message::Response {
                error: ErrorCode::ITEM_NOT_FOUND,
                payload: Payload::Empty,
            };
        }
    };

    info!("Get {} info", item_ref);

    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Json(serde_json::to_value(item).unwrap()),
    }
}

pub(super) fn items_request(server_info: &SharedServer) -> Message {
    info!("Get all items info");
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Json(
            serde_json::to_value(&server_info.lock().unwrap().world.items).unwrap(),
        ),
    }
}
