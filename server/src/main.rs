use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
enum MessageType {
    COMMAND,
    RESPONSE,
    EVENT,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
enum EventType {
    NONE,
}

#[derive(Serialize, Deserialize)]
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

impl Message {
    fn default() -> Self {
        Message {
            message: MessageType::COMMAND,
            command_line: String::new(),
            response_line: String::new(),
            event_line: String::new(),
            command_name: String::new(),
            args: Vec::new(),
            error_response: ErrorCode::NONE,
            error_code: ErrorCode::NONE.code(),
            event_type: EventType::NONE,
            event_data: String::new(),
        }
    }

    fn parse(str: String) -> Self {
        serde_json::from_str(&str).unwrap_or_else(|_| Message::default())
    }

    fn to_str(message: Self) -> String {
        serde_json::to_string(&message).unwrap_or_default() + "\n"
    }
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

fn connect_request(request: Message, server_info: &Arc<Mutex<ServerInfo>>) -> Message {
    if request.args.len() != 1
        || server_info
            .lock()
            .unwrap()
            .players
            .contains(&request.args[0])
    {
        return Message {
            message: MessageType::RESPONSE,
            error_response: ErrorCode::NAME_IN_USE,
            error_code: ErrorCode::NAME_IN_USE.code(),
            ..request
        };
    }
    server_info
        .lock()
        .unwrap()
        .players
        .push(request.args[0].clone());
    println!("{}", server_info.lock().unwrap().players.last().unwrap());
    return Message {
        message: MessageType::RESPONSE,
        error_response: ErrorCode::NONE,
        error_code: ErrorCode::NONE.code(),
        ..request
    };
}

fn parse_command(line: &str) -> Message {
    let mut parts = line.split_whitespace();
    let command_name = parts.next().unwrap_or("").to_string();
    let args: Vec<String> = parts.map(String::from).collect();

    Message {
        message: MessageType::COMMAND,
        command_line: line.to_string(),
        command_name,
        args,
        ..Message::default()
    }
}

fn handle_request(request: Message, server_info: &Arc<Mutex<ServerInfo>>) -> Message {
    if request.message != MessageType::COMMAND {
        return Message::default();
    };
    match request.command_name.to_uppercase().as_str() {
        "CONNECT" => connect_request(request, server_info),
        _ => Message::default(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_info: Arc<Mutex<ServerInfo>> = Arc::new(Mutex::new(ServerInfo::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        if let Err(_e) = socket
            .write_all("Server → Client: OK hello proto=1\n".as_bytes())
            .await
        {
            eprintln!("failed to connect establish TCP connection with server");
        }

        let (read_half, mut write_half) = socket.into_split();
        let mut reader = BufReader::new(read_half);
        let mut line = String::new();

        let server_info_copy = Arc::clone(&server_info);

        tokio::spawn(async move {
            // In a loop, read data from the socket and write the data back.
            loop {
                line.clear();
                let n = reader.read_line(&mut line).await?;
                if n == 0 {
                    break;
                }
                let line = parse_command(line.as_str());
                let response = handle_request(line, &server_info_copy);
                let _ = write_half
                    .write_all(Message::to_str(response).as_bytes())
                    .await;
            }

            Ok::<(), std::io::Error>(())
        });
    }
}
