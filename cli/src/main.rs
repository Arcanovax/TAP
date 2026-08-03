mod draw_functions;
mod enums;
mod global_functions;
mod structures;

use crate::enums::states::States;
use crate::structures::world::World;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use std::io::{Error, stdout};
use std::sync::mpsc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

async fn network_task(tx: mpsc::Sender<String>, mut rx: tokio::sync::mpsc::Receiver<String>) {
	let addr: String = std::env::args()
            .nth(1)
            .unwrap_or_else(|| "127.0.0.1:8080".into());
    let stream = match TcpStream::connect(addr).await {
        Ok(s) => s,
        Err(_) => return,
    };
    let (half_reader, mut writer) = stream.into_split();

    let read_task = tokio::spawn(async move {
        let mut reader = BufReader::new(half_reader);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    if tx.send(line.clone()).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if writer.write_all(msg.as_bytes()).await.is_err() {
                break;
            }
        }
    });

    let _ = tokio::join!(read_task, write_task);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (tx_to_game, rx_from_serv) = mpsc::channel::<String>();
    let (tx_to_serv, rx_from_game) = tokio::sync::mpsc::channel::<String>(32);
    tokio::spawn(network_task(tx_to_game, rx_from_game));

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        let _ = execute!(std::io::stdout(), DisableMouseCapture);
        original_hook(panic_info);
    }));

    let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
    let mut world: World = World::new(tx_to_serv, rx_from_serv);
    let result = world.run(&mut terminal);
    execute!(stdout(), DisableMouseCapture)?;
    ratatui::restore();
    if let States::ServerError(msg) = world.state {
        println!("Server Error: {}", msg)
    }
    result
}
