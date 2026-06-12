mod structures;
mod draw_functions;
mod global_functions;
mod enums;

use std::io::{Error, stdout};
use std::sync::mpsc;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::enums::states::States;
use crate::structures::world::World;

async fn network_task(tx: mpsc::Sender<String>, mut rx: tokio::sync::mpsc::Receiver<String>) {
    let stream = match TcpStream::connect("127.0.0.1:8080").await {
		Ok(s) => s,
		Err(_) => return,
	};
    let (mut reader, mut writer) = stream.into_split();


    let read_task = tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            match reader.read(&mut buf).await{
				Ok(n) if n > 0 => {
					tx.send(String::from_utf8_lossy(&buf[..n]).to_string()).ok();
				}
				_ => break,
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
        let _ = ratatui::restore();
        let _ = execute!(std::io::stdout(), DisableMouseCapture);
        original_hook(panic_info);
    }));

	let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
	let mut world: World = World::new(tx_to_serv, rx_from_serv);
    let result = world.run(&mut terminal);
    execute!(stdout(), DisableMouseCapture)?;
	ratatui::restore();
	match world.state {
		States::ServerError(msg) => println!("Server Error: {}", msg),
		_ => {}
	}
	result
}

