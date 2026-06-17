use crate::{protocol::Message, state::SharedServer, structures::enums::error::ErrorCode};

#[cfg(test)]
mod tests;

pub(super) fn item_request(server_info: &SharedServer, args: &Vec<String>) -> Message {
    let binding = server_info.lock().unwrap();

    let mut item_ref = args.join(" ");
    if let Some(reference) = binding.world.name_to_ref.get(&item_ref.to_lowercase()) {
        item_ref = reference.clone();
    }

    let item = match binding.world.items.get(&item_ref) {
        Some(item) => item,
        None => {
            return Message::Response {
                error: ErrorCode::ITEM_NOT_FOUND,
                data: None,
            };
        }
    };

    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(serde_json::to_value(item).unwrap()),
    }
}
