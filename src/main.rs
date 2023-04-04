use std::error::Error;
use std::{env};
use std::str::FromStr;
use tokio::net::TcpListener;

use gyrogun_server::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let addr = "0.0.0.0:11076";

    let listener = TcpListener::bind(&addr).await?;

    let client_count = i32::from_str(args.get(1).unwrap_or(&String::from("1")).as_str())?;

    let mut pos_rxs: Vec<tokio::sync::watch::Receiver<(f32, f32)>> = vec![];

    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::channel(128);

    println!("Server up, waiting for {client_count} clients");

    for i in 0..client_count {
        let (socket, addr) = listener.accept().await.unwrap();

        let pos_rx = client::handle(socket, addr, i as u32, msg_tx.clone()).await;

        pos_rxs.push(pos_rx);
    }

    gyrogun_server::display::launch(pos_rxs);

    // Somehow move this to game logic using game::Message
    let mut disconnect_count = 0;

    loop {
        let (client, msg)= msg_rx.recv().await.unwrap();
        println!("Client #{client}: {:?}", msg);

        if let client::Message::Disconnect = msg {
            disconnect_count += 1;

            if disconnect_count == client_count {
                println!("All clients disconnected, exiting");
                break;
            }
        }
    }

    Ok(())
}


