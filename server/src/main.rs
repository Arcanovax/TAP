use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::ErrorCode::NONE;

enum MessageType {
    COMMAND,
    RESPONSE,
    EVENT,
}

#[allow(non_camel_case_types)]
enum ErrorCode {
    NAME_IN_USE,
    NO_EXIT,
    NOT_IN_GROUP,
    ALREADY_IN_GROUP,
    ITEM_NOT_FOUND,
    ITEM_NOT_IN_INVENTORY,
    NPC_NOT_FOUND,
    NPC_NOT_HOSTILE,
    NO_QUEST_AVAILABLE,
    CONNECTION_FAILED,
    SEND_FAILED,
    NONE,
}

impl ErrorCode {
    fn code(&self) -> u16 {
        match self {
            ErrorCode::NAME_IN_USE => 201,
            ErrorCode::NO_EXIT => 301,
            ErrorCode::NOT_IN_GROUP => 401,
            ErrorCode::ALREADY_IN_GROUP => 402,
            ErrorCode::ITEM_NOT_FOUND
            | ErrorCode::ITEM_NOT_IN_INVENTORY
            | ErrorCode::NPC_NOT_FOUND => 404,
            ErrorCode::NPC_NOT_HOSTILE => 405,
            ErrorCode::NO_QUEST_AVAILABLE => 406,
            ErrorCode::CONNECTION_FAILED => 900,
            ErrorCode::SEND_FAILED => 901,
            ErrorCode::NONE => 0,
        }
    }
}

enum EventType {
    NONE,
}

struct Message {
    message: MessageType,
    command_line: String,
    response_line: String,
    event_line: String,
    command_name: String,
    args: Vec<String>,
    error_response: ErrorCode,
    error_code: u16,
    event_type: EventType,
    event_data: String,
}

struct ServerInfo {
    players: Vec<String>,
}

impl ServerInfo {
    fn new() -> Self {
        ServerInfo {
            players: Vec::new(),
        }
    }
}

fn connect_request(request: Message, server_info: &ServerInfo) -> Message {
    if server_info.players.contains(&request.args[0]) {
        return Message {
            message: MessageType::RESPONSE,
            error_response: ErrorCode::NAME_IN_USE,
            error_code: ErrorCode::NAME_IN_USE.code(),
            ..request
        };
    }
    return Message {
        message: MessageType::RESPONSE,
        error_response: ErrorCode::NONE,
        error_code: ErrorCode::NONE.code(),
        ..request
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_info = ServerInfo::new();
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        if let Err(_e) = socket
            .write_all("Server → Client: OK hello proto=1\n".as_bytes())
            .await
        {
            eprintln!("failed to connect establish TCP connection with server");
        }
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let _ = match std::str::from_utf8(&buf[0..n]) {
                    Ok(text) => {
                        let text = text.trim();

                        if text == "CONNECT" {
                            connect_request(
                                Message {
                                    message: MessageType::COMMAND,
                                    command_line: String::from("CONNECT Jonh\n"),
                                    response_line: String::new(),
                                    event_line: String::new(),
                                    command_name: String::from("CONNECT"),
                                    args: vec![String::from("John")],
                                    error_response: ErrorCode::NONE,
                                    error_code: ErrorCode::NONE.code(),
                                    event_type: EventType::NONE,
                                    event_data: String::new(),
                                },
                                &server_info,
                            );
                        } else {
                            let _ = socket.write_all("Command Not found\n".as_bytes()).await;
                        }
                    }
                    Err(_) => eprintln!("Command not utf8"),
                };
            }
        });
    }
}
