use std::f32::consts::PI;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, watch};
use crate::client::init::InitPhase;
use crate::client::raw_message::RawMessage;

pub mod fake;
pub mod init;
mod raw_message;

type SensorData = (f32, f32, f32);
pub type PosCoord = (f32, f32);
type ShooterCoord = (f32, f32, f32);

#[derive(Debug)]
pub enum Message {
    Click(PosCoord),
    DoubleClick(PosCoord),
    Disconnect,
}

pub async fn handle(
    mut socket: TcpStream, addr: SocketAddr, index: u32,
    msg_tx: mpsc::Sender<(u32, Message)>,
    mut next_phase_rx: watch::Receiver<Option<InitPhase>>,
    done_phase_tx: watch::Sender<Option<InitPhase>>,
    window_size: (f32, f32),
) -> watch::Receiver<PosCoord> {
    println!("Handling connection of client #{index}, {addr}");

    let (pos_tx, pos_rx) = watch::channel((0.0, 0.0));

    tokio::spawn(async move {
        let mut phase;
        let mut init_data = init::InitData::new(window_size);
        let mut shooter: ShooterCoord = (0.0, 0.0, 0.0);

        loop {
            let raw_message = RawMessage::read(&mut socket).await;
            phase = *next_phase_rx.borrow();

            if let None = &phase {
                done_phase_tx.send(None).unwrap();
            }

            if let Some(raw_message) = raw_message {
                if let Some(p) = &phase {
                    if let RawMessage::Click(data) = raw_message {
                        match p {
                            InitPhase::WaitMonitor => {
                                init_data.set_monitor(data);
                                println!("Wait monitor {index} done")
                            }
                            InitPhase::WaitFirstPoint => {
                                init_data.set_first_point(data);
                                println!("Wait first point {index} done")
                            }
                            InitPhase::WaitSecondPoint => {
                                init_data.set_second_point(data);
                                println!("Wait second point {index} done")
                            }
                            InitPhase::Finalize => {
                                shooter = shooter_pos(&init_data);
                                println!("Wait finalize {index} done")
                            }
                        }
                        done_phase_tx.send(Some(*p)).unwrap();
                    }
                } else {
                    if let RawMessage::Position(data) = raw_message {
                        let pos = screen_pos(&init_data, data, shooter);
                        pos_tx.send(pos).unwrap();
                    } else if let RawMessage::Click(data) = raw_message {
                        let pos = screen_pos(&init_data, data, shooter);
                        msg_tx.send((index, Message::Click(reverse_fix_pos(pos, window_size)))).await.unwrap();
                    }
                }

                if let RawMessage::DoubleClick(_) = raw_message { // Temporary
                    phase = Some(init::InitPhase::WaitMonitor);
                }
            } else {
                msg_tx.send((index, Message::Disconnect)).await.unwrap();
            }
        }
    });

    pos_rx
}

/*
    Refer to fake.rs fix_pos
    an reverse of the function
 */
fn reverse_fix_pos(pos: PosCoord, window_size: (f32, f32)) -> PosCoord {
    let (width, height) = window_size;
    let (x, y) = pos;

    (x + width / 2.0, height / 2.0 - y)
}

fn shooter_pos(init_data: &init::InitData) -> ShooterCoord {
    let monitor_yaw = init_data.monitor().0;
    let a_yaw = init_data.first_point().0;
    let b_yaw = init_data.second_point().0;
    let avg_pitch = init_data.first_point().1; // Come up with a better value later


    let a_tan = ((monitor_yaw - a_yaw + 90.0) * PI / 180.0).tan();
    let b_tan = ((monitor_yaw - b_yaw + 90.0) * PI / 180.0).tan();
    let h_tan = (avg_pitch * PI / 180.0).tan();
    println!("a_tan {a_tan} b_tan {b_tan}");

    let x = (a_tan + b_tan) / (a_tan - b_tan) * (-init_data.window_size().1 / 2.0);
    let y = a_tan * (x + init_data.window_size().1 / 2.0);
    let h = (x * x + y * y).sqrt() * h_tan;

    return (x, y, h);
}

fn screen_pos(init_data: &init::InitData, curr_data: SensorData,  shooter_pos: ShooterCoord) -> PosCoord {
    let monitor_yaw = init_data.monitor().0;
    let monitor_pitch = init_data.monitor().1;
    let curr_yaw = curr_data.0;
    let curr_pitch = curr_data.1;

    let xy_tan = ((monitor_yaw - curr_yaw + 90.0) * PI / 180.0).tan();
    let (x, y, h) = shooter_pos;

    let my_tan = ((90.0 - monitor_pitch) * PI / 180.0).tan();
    let my_cos = (monitor_pitch * PI / 180.0).cos();
    let cy_tan = (curr_pitch * PI / 180.0).tan();

    let retx = x - y / xy_tan;
    let rety = ((x * x + y * y).sqrt() * -cy_tan + h) / (my_tan + cy_tan) * my_tan / my_cos;

    return (retx, rety);
}
