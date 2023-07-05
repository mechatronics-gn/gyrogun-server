use std::error::Error;
use std::{env, thread};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

use gyrogun_server::client;
use gyrogun_server::game::{Game};
use gyrogun_server::game::balloon_game::BalloonGame;
use gyrogun_server::texture::TextureStore;

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

    let (objects_tx, objects_rx) = tokio::sync::watch::channel(vec![]);
    let (time_tx, time_rx) = tokio::sync::watch::channel(0);
    let (bg_color_tx, bg_color_rx) = tokio::sync::watch::channel(macroquad::color::WHITE);
    let (mut sounds_tx, sounds_rx) = std::sync::mpsc::channel();

    if client_count > 0 {
        let mut pos_rxs: Vec<tokio::sync::watch::Receiver<(f32, f32)>> = vec![];
        for i in 0..client_count {
            let (socket, addr) = listener.accept().await.unwrap();

            let pos_rx = client::handle(socket, addr, i as u32, msg_tx.clone(), window_size).await;

            pos_rxs.push(pos_rx);
        }

        gyrogun_server::display::launch(pos_rxs, window_size, None, objects_rx, time_rx, bg_color_rx, sounds_rx);
    } else {
        let fake_client_count = -client_count;

        let (fake_input_tx, fake_input_rx) = std::sync::mpsc::channel();

        let pos_rxs = client::fake::handle(fake_input_rx, msg_tx, fake_client_count, window_size);

        gyrogun_server::display::launch(pos_rxs, window_size,Some(fake_input_tx), objects_rx, time_rx, bg_color_rx, sounds_rx);
    }


    // Somehow move this to game logic using game::Message
    let mut disconnect_count = 0;
    let game_duration = 9000;

    loop {

        let mut game = BalloonGame::new(window_size, client_count.abs() as u32, game_duration);
        let mut time = 0;

        loop {
            if time > game_duration {
                break;
            }
            game.on_time(time);

            loop {
                let try_recv = msg_rx.try_recv();
                if let Err(_) = try_recv {
                    break;
                } else if let Ok((client, msg)) = try_recv {
                    if let client::Message::Disconnect = msg {
                        disconnect_count += 1;

                        if disconnect_count == client_count {
                            println!("All clients disconnected, exiting");
                            break;
                        }
                    }

                    game.on_message(client, msg, time, &mut sounds_tx);
                }
            }
            time_tx.send(time).ok();

            bg_color_tx.send(game.background_color(time)).ok();

            if game.was_objects_updated() {
                objects_tx.send(game.objects(time)).ok();
            }

            time += 1;
            thread::sleep(Duration::from_millis(10));
        }

    }

}


