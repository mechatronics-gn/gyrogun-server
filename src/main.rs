use std::error::Error;
use std::{env, thread};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::net::TcpListener;

use gyrogun_server::client;
use gyrogun_server::client::init::InitPhase;
use gyrogun_server::client::position_manager::PositionManager;
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
    let server_addr = args.get(4).and_then(|x| Some(x.as_str())).unwrap_or("0.0.0.0:11076");

    let window_size = (width, height);

    let listener = TcpListener::bind(&server_addr).await?;
    let mut pos_man = PositionManager::new();

    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::channel(128);

    println!("Server up, waiting for {client_count} clients");

    let (objects_tx, objects_rx) = tokio::sync::watch::channel(vec![]);
    let (time_tx, time_rx) = tokio::sync::watch::channel(0);
    let (bg_color_tx, bg_color_rx) = tokio::sync::watch::channel(macroquad::color::WHITE);
    let (mut sounds_tx, sounds_rx) = std::sync::mpsc::channel();


    let mut next_phase_txs = HashMap::new();
    let mut done_phase_rxs = HashMap::new();
    if client_count > 0 {
        let mut pos_rxs: HashMap<u32, tokio::sync::watch::Receiver<(f32, f32)>> = HashMap::new();
        let mut successes = 0;
        while successes < client_count {
            let (tcp_sock, addr) = listener.accept().await.unwrap();
            let (init_data_tx, pos_rx) = pos_man.register(addr);

            let (next_phase_tx, next_phase_rx) = tokio::sync::watch::channel(None);
            let (done_phase_tx, done_phase_rx) = tokio::sync::watch::channel(None);
            let index = client::handle(tcp_sock, addr, msg_tx.clone(), next_phase_rx, done_phase_tx, init_data_tx, window_size).await;

            if let Some(index) = index {
                pos_rxs.insert(index, pos_rx);
                next_phase_txs.insert(index, next_phase_tx);
                done_phase_rxs.insert(index, done_phase_rx);
                successes += 1;
            }
        }

        let server_addr = String::from(server_addr);
        tokio::spawn(async move {
            pos_man.run(&server_addr).await;
        });
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
            println!("Starting initialize");
            let mut init_phase = Some(InitPhase::WaitMonitor);
            let mut tutorial = Tutorial::new(init_phase.unwrap());
            let mut time = 0;
            loop {
                for (_, recv) in &mut done_phase_rxs {
                    recv.borrow_and_update();
                }

                for (_, send) in &next_phase_txs {
                    send.send(init_phase).unwrap();
                }

                println!("sent next phase tx {:?}", init_phase);

                let mut x: Vec<(u32, InitPhase)>;
                loop {
                    let mut flag = false;
                    for (idx, recv) in &done_phase_rxs {
                        let x = recv.borrow();
                        if x.is_none() {
                            flag = true;
                            tutorial.update_init_state(*idx as i32, false);
                        }
                    }

                    if !flag {
                        x = done_phase_rxs.iter().map(|(idx, x)| (*idx, x.borrow().unwrap())).collect();
                        let mut flag = true;
                        for (idx, x) in &x {
                            if init_phase.map_or(false, |y| *x == y ) {
                                tutorial.update_init_state(*idx as i32, true);
                            } else {
                                flag = false;
                                tutorial.update_init_state(*idx as i32, false);
                            }
                        }
                        if flag {
                            break;
                        }
                    }

                    single_frame(&mut tutorial, &mut time, &mut disconnect_count, client_count, &mut msg_rx, &mut sounds_tx, &time_tx, &bg_color_tx, &objects_tx);
                    spin_sleep::sleep(Duration::from_millis(10));
                }

                match x[0].1 {
                    InitPhase::WaitMonitor => {
                        println!("Initphase is waitmonitor and going to exit here");
                        init_phase = Some(InitPhase::WaitFirstPoint);
                        tutorial.update_init_phase(InitPhase::WaitFirstPoint, time, 80);
                    }
                    InitPhase::WaitFirstPoint => {
                        init_phase = Some(InitPhase::WaitSecondPoint);
                        tutorial.update_init_phase(InitPhase::WaitSecondPoint, time, 80);
                    }
                    InitPhase::WaitSecondPoint => {
                        init_phase = Some(InitPhase::Finalize);
                        tutorial.update_init_phase(InitPhase::Finalize, time, 80);
                    }
                    InitPhase::Finalize => {
                        init_phase = None;
                        let time_target = time + 100;
                        while time <= time_target {
                            single_frame(&mut tutorial, &mut time, &mut disconnect_count, client_count, &mut msg_rx, &mut sounds_tx, &time_tx, &bg_color_tx, &objects_tx);
                            spin_sleep::sleep(Duration::from_millis(10));
                        }

                        for (_, send) in &next_phase_txs {
                            send.send(init_phase).unwrap();
                        }

                        break;
                    }
                }
                let time_target = time + 100;
                while time <= time_target {
                    single_frame(&mut tutorial, &mut time, &mut disconnect_count, client_count, &mut msg_rx, &mut sounds_tx, &time_tx, &bg_color_tx, &objects_tx);
                    spin_sleep::sleep(Duration::from_millis(10));
                }
            }
        }

        println!("Getting into game");

        let game_duration = 6000;
        let mut game = BalloonGame::new(window_size, client_count.abs() as u32, game_duration);
        let mut time = 0;

        while time <= game_duration {
            single_frame(&mut game, &mut time, &mut disconnect_count, client_count, &mut msg_rx, &mut sounds_tx, &time_tx, &bg_color_tx, &objects_tx);
            spin_sleep::sleep(Duration::from_millis(10));
        }

        let results_duration = 1500;
        let mut results = BalloonResults::from(window_size, &game);
        let mut time = 0;

        while time <= results_duration {
            single_frame(&mut results, &mut time, &mut disconnect_count, client_count, &mut msg_rx, &mut sounds_tx, &time_tx, &bg_color_tx, &objects_tx);
            spin_sleep::sleep(Duration::from_millis(10));
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

