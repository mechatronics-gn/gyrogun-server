use std::error::Error;
use std::{env, thread};
use std::os::unix::raw::time_t;
use std::str::FromStr;
use std::time::Duration;
use macroquad::window::next_frame;
use tokio::net::TcpListener;

use gyrogun_server::client;
use gyrogun_server::client::init::InitPhase;
use gyrogun_server::game::{Game};
use gyrogun_server::game::balloon_game::BalloonGame;
use gyrogun_server::game::balloon_results::BalloonResults;
use gyrogun_server::game::object::ObjectWrapper;
use gyrogun_server::game::tutorial::Tutorial;
use gyrogun_server::sound::SoundType;

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


    let mut next_phase_txs = vec![];
    let mut done_phase_rxs = vec![];
    if client_count > 0 {
        let mut pos_rxs: Vec<tokio::sync::watch::Receiver<(f32, f32)>> = vec![];
        for i in 0..client_count {
            let (socket, addr) = listener.accept().await.unwrap();

            let (next_phase_tx, next_phase_rx) = tokio::sync::watch::channel(None);
            let (done_phase_tx, done_phase_rx) = tokio::sync::watch::channel(None);
            let pos_rx = client::handle(socket, addr, i as u32, msg_tx.clone(), next_phase_rx, done_phase_tx, window_size).await;

            pos_rxs.push(pos_rx);
            next_phase_txs.push(next_phase_tx);
            done_phase_rxs.push(done_phase_rx);
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

    loop {
        // Initialize
        if client_count > 0 {
            let mut init_phase = Some(InitPhase::WaitMonitor);
            let mut tutorial = Tutorial::new(init_phase.unwrap());
            let mut time = 0;
            loop {
                for i in &next_phase_txs {
                    i.send(init_phase).unwrap();
                }

                let mut x: Vec<InitPhase>;
                loop {
                    x = done_phase_rxs.iter().map(|x| x.borrow().unwrap()).collect();
                    if x.iter().all(|t| *t == x[0]) {
                        break;
                    }

                    single_frame(&mut tutorial, &mut time, &mut disconnect_count, client_count, &mut msg_rx, &mut sounds_tx, &time_tx, &bg_color_tx, &objects_tx);
                    thread::sleep(Duration::from_millis(10));
                }

                match x[0] {
                    InitPhase::WaitMonitor => {
                        init_phase = Some(InitPhase::WaitFirstPoint);
                        tutorial.update_init_phase(InitPhase::WaitFirstPoint, time, 100);
                        thread::sleep(Duration::from_secs(1));
                    }
                    InitPhase::WaitFirstPoint => {
                        init_phase = Some(InitPhase::WaitSecondPoint);
                        tutorial.update_init_phase(InitPhase::WaitSecondPoint, time, 100);
                        thread::sleep(Duration::from_secs(1));
                    }
                    InitPhase::WaitSecondPoint => {
                        init_phase = Some(InitPhase::Finalize);
                        tutorial.update_init_phase(InitPhase::Finalize, time, 100);
                        thread::sleep(Duration::from_secs(1));
                    }
                    InitPhase::Finalize => {
                        init_phase = None;
                        thread::sleep(Duration::from_secs(1));

                        for i in &next_phase_txs {
                            i.send(init_phase).unwrap();
                        }

                        break;
                    }
                }
            }
        }

        let game_duration = 6000;
        let mut game = BalloonGame::new(window_size, client_count.abs() as u32, game_duration);
        let mut time = 0;

        while time <= game_duration {
            single_frame(&mut game, &mut time, &mut disconnect_count, client_count, &mut msg_rx, &mut sounds_tx, &time_tx, &bg_color_tx, &objects_tx);
            thread::sleep(Duration::from_millis(10));
        }

        let results_duration = 1500;
        let mut results = BalloonResults::from(window_size, &game);
        let mut time = 0;

        while time <= results_duration {
            single_frame(&mut results, &mut time, &mut disconnect_count, client_count, &mut msg_rx, &mut sounds_tx, &time_tx, &bg_color_tx, &objects_tx);
            thread::sleep(Duration::from_millis(10));
        }
    }

}

fn single_frame<T: Game>(
    game: &mut T,
    time: &mut u32,
    disconnect_count: &mut i32,
    client_count: i32,
    msg_rx: &mut tokio::sync::mpsc::Receiver<(u32, client::Message)>,
    sounds_tx: &mut std::sync::mpsc::Sender<SoundType>,
    time_tx: &tokio::sync::watch::Sender<u32>,
    bg_color_tx: &tokio::sync::watch::Sender<macroquad::color::Color>,
    objects_tx: &tokio::sync::watch::Sender<Vec<ObjectWrapper>>,
) {
    game.on_time(*time);

    loop {
        let try_recv = msg_rx.try_recv();
        if let Err(_) = try_recv {
            break;
        } else if let Ok((client, msg)) = try_recv {
            if let client::Message::Disconnect = msg {
                *disconnect_count += 1;

                if *disconnect_count == client_count {
                    println!("All clients disconnected, exiting");
                    break;
                }
            }

            game.on_message(client, msg, *time, sounds_tx);
        }
    }
    time_tx.send(*time).ok();

    bg_color_tx.send(game.background_color(*time)).ok();

    if game.was_objects_updated() {
        objects_tx.send(game.objects(*time)).ok();
    }

    *time += 1;
}

