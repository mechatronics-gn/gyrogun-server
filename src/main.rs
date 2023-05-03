use std::error::Error;
use std::{env};
use std::str::FromStr;
use tokio::net::TcpListener;

use gyrogun_server::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let client_count = args.get(1).and_then(|x| i32::from_str(x).ok()).unwrap_or(1);
    let width = args.get(2).and_then(|x| f32::from_str(x).ok()).unwrap_or(1920.0);
    let height = args.get(3).and_then(|x| f32::from_str(x).ok()).unwrap_or(1080.0);
    let addr = args.get(4).and_then(|x| Some(x.as_str())).unwrap_or("0.0.0.0:11076");

    let window_size = (width, height);

    let listener = TcpListener::bind(&addr).await?;


    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::channel(128);

    println!("Server up, waiting for {client_count} clients");

    if client_count > 0 {
        let mut pos_rxs: Vec<tokio::sync::watch::Receiver<(f32, f32)>> = vec![];
        for i in 0..client_count {
            let (socket, addr) = listener.accept().await.unwrap();

            let pos_rx = client::handle(socket, addr, i as u32, msg_tx.clone(), window_size).await;

            pos_rxs.push(pos_rx);
        }

        gyrogun_server::display::launch(pos_rxs, window_size, None);
    } else {
        let fake_client_count = -client_count;

        let (fake_input_tx, fake_input_rx) = std::sync::mpsc::channel();

        let pos_rxs = client::fake::handle(fake_input_rx, msg_tx, fake_client_count, window_size);

        gyrogun_server::display::launch(pos_rxs, window_size, Some(fake_input_tx));
    }


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


