use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::SharedServer;
use std::net::SocketAddr;

pub(super) fn group_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }

    match args[0].to_uppercase().as_str() {
        "CREATE" => group_create_request(server_info, peer_addr, args),
        "LEAVE" => group_leave_request(server_info, peer_addr),
        "INVITE" => group_invite_request(args, server_info, peer_addr),
        "JOIN" => group_join_request(server_info, peer_addr),
        "LIST" => group_list_request(server_info, peer_addr),
        _ => Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        },
    }
}

fn group_create_request(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    args: &Vec<String>,
) -> Message {
    let group_name: String;
    if args.len() <= 1 {
        group_name = match server_info.lock().unwrap().get_player(peer_addr) {
            Ok(player) => player.name.clone() + "'s group",
            Err(code) => {
                return Message::Response {
                    error: code,
                    data: None,
                };
            }
        };
    } else {
        group_name = args[1..].join(" ");
    }

    server_info
        .lock()
        .unwrap()
        .try_create_group(peer_addr, group_name.as_str())
        .into()
}

fn group_leave_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    server_info
        .lock()
        .unwrap()
        .try_leave_group(peer_addr)
        .into()
}

fn group_invite_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if args.len() != 2 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }

    server_info
        .lock()
        .unwrap()
        .try_invite_group(args[1].clone(), peer_addr)
        .into()
}

fn group_join_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    server_info.lock().unwrap().try_join_group(peer_addr).into()
}

fn group_list_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    server_info
        .lock()
        .unwrap()
        .try_get_group_list(peer_addr)
        .map(|list| serde_json::to_string(&list).unwrap())
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{addr, connect, err, test_server};

    // GROUP CREATE TESTS

    #[test]
    fn group_create_without_connected_return_invalid_command() {
        let server = test_server();
        let addr = addr(10101);
        let result = group_create_request(&server, addr, &vec![]);
        assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
    }

    #[test]
    fn group_create_with_connected_return_success() {
        let server = test_server();
        let addr = addr(10101);
        connect(&server, addr, "test_user");
        let result = group_create_request(&server, addr, &vec![]);
        assert_eq!(result, err(ErrorCode::SUCCESS));
    }

    #[test]
    fn group_create_with_group_return_already_in_group() {
        let server = test_server();
        let addr = addr(10101);
        connect(&server, addr, "test_user");
        group_create_request(&server, addr, &vec![]);
        let result = group_create_request(&server, addr, &vec![]);
        assert_eq!(result, err(ErrorCode::ALREADY_IN_GROUP));
    }

    #[test]
    fn group_create_with_args_return_success() {
        let server = test_server();
        let addr = addr(10101);
        connect(&server, addr, "test_user");
        let result = group_create_request(
            &server,
            addr,
            &vec![
                "test".to_string(),
                "custom".to_string(),
                "group".to_string(),
            ],
        );
        assert_eq!(result, err(ErrorCode::SUCCESS));
    }
}
