use crate::error::ErrorCode;
use crate::protocol::{ChatScope, EventType, Message};
use crate::state::SharedServer;
use std::net::SocketAddr;
use tracing::info;

pub(super) fn chat_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if !server_info.lock().unwrap().is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        };
    }
    if args.len() <= 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let scope = &args[0];
    let body = args[1..].join(" ") + "\n";
    let mut binding = server_info.lock().unwrap();

    let sender_name = match binding.get_player(peer_addr) {
        Ok(player) => player.name.clone(),
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };

    let receivers = match scope.to_uppercase().as_str() {
        "GLOBAL" => binding.get_global_receivers(peer_addr),
        "GROUP" => match binding.get_group_receivers(peer_addr) {
            Ok(receivers) => receivers,
            Err(code) => {
                return Message::Response {
                    error: code,
                    data: None,
                };
            }
        },
        "ROOM" => match binding.get_room_receivers(peer_addr) {
            Ok(receivers) => receivers,
            Err(code) => {
                return Message::Response {
                    error: code,
                    data: None,
                };
            }
        },
        _ => {
            return Message::Response {
                error: ErrorCode::INVALID_ARGS,
                data: None,
            };
        }
    };
    let chat_scope = match scope.parse::<ChatScope>() {
        Ok(scope) => scope,
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };
    for con in receivers {
        let _ = con.tx.send(Message::Event(EventType::CHAT {
            body: body.clone(),
            sender: sender_name.clone(),
            scope: chat_scope.clone(),
        }));
    }
    info!("Send {} scoped chat: {}", scope.to_uppercase(), body);
    return Message::Response {
        error: ErrorCode::SUCCESS,
        data: None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{addr, connect, err, test_server};

    #[test]
    fn chat_without_connection_returns_invalid_command() {
        let server = test_server();
        let result = chat_request(
            &vec!["GLOBAL".to_string(), "hi".to_string()],
            &server,
            addr(1),
        );
        assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
    }

    #[test]
    fn chat_without_body_returns_invalid_args() {
        let server = test_server();
        connect(&server, addr(1), "alice");
        let result = chat_request(&vec!["GLOBAL".to_string()], &server, addr(1));
        assert_eq!(result, err(ErrorCode::INVALID_ARGS));
    }

    #[test]
    fn chat_with_unknown_scope_returns_invalid_args() {
        let server = test_server();
        connect(&server, addr(1), "alice");
        let result = chat_request(
            &vec!["WHISPER".to_string(), "hi".to_string()],
            &server,
            addr(1),
        );
        assert_eq!(result, err(ErrorCode::INVALID_ARGS));
    }

    #[test]
    fn chat_group_scope_without_group_returns_not_in_group() {
        let server = test_server();
        connect(&server, addr(1), "alice");
        let result = chat_request(
            &vec!["GROUP".to_string(), "hi".to_string()],
            &server,
            addr(1),
        );
        assert_eq!(result, err(ErrorCode::NOT_IN_GROUP));
    }

    #[test]
    fn chat_global_broadcasts_to_others_not_sender() {
        let server = test_server();
        let mut rx_alice = connect(&server, addr(1), "alice");
        let mut rx_bob = connect(&server, addr(2), "bob");

        let result = chat_request(
            &vec!["GLOBAL".to_string(), "hello world".to_string()],
            &server,
            addr(1),
        );

        assert_eq!(result, err(ErrorCode::SUCCESS));
        assert_eq!(
            rx_bob.try_recv().expect("bob should receive the chat"),
            Message::Event(EventType::CHAT {
                body: "hello world\n".to_string(),
                sender: "alice".to_string(),
                scope: ChatScope::GLOBAL,
            })
        );
        assert!(
            rx_alice.try_recv().is_err(),
            "the sender must not receive its own chat"
        );
    }
}
