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
use crate::structures::world::World;

async fn network_task(tx: mpsc::Sender<String>, mut rx: tokio::sync::mpsc::Receiver<String>) {
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (mut reader, mut writer) = stream.into_split();


    let read_task = tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            let n = reader.read(&mut buf).await.unwrap();
            if n == 0 { break; }
            tx.send(String::from_utf8_lossy(&buf[..n]).to_string()).ok();
        }
    });

    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            writer.write_all(msg.as_bytes()).await.unwrap();
        }
    });

    let _ = tokio::join!(read_task, write_task);
}
#[tokio::main]
async fn main() -> Result<(), Error> {

	let (tx_to_game, rx_from_serv) = mpsc::channel::<String>();
	let (tx_to_serv, rx_from_game) = tokio::sync::mpsc::channel::<String>(32);
	std::thread::spawn(move || {
			tokio::runtime::Runtime::new()
				.unwrap()
				.block_on(network_task(tx_to_game, rx_from_game));
		});
	
	let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
	let mut world: World = World::new(tx_to_serv, rx_from_serv);
    let result = world.run(&mut terminal);
    execute!(stdout(), DisableMouseCapture)?;
	ratatui::restore();
	result
}

