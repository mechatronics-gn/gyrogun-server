use std::collections::HashMap;
use tokio::sync::watch;
use super::PosCoord;

pub enum RawMessage {
    Hover(PosCoord),
    LeftClick(PosCoord),
    RightClick(PosCoord),
    MiddleClick,
}

pub fn handle(
    fake_input_rx: std::sync::mpsc::Receiver<RawMessage>,
    msg_tx: tokio::sync::mpsc::Sender<(u32, super::Message)>,
    count: i32,
    window_size: (f32, f32),
) -> HashMap<u32, watch::Receiver<PosCoord>> {
    let mut ret = HashMap::new();
    let mut pos_txs = vec![];

    for i in 0..count {
        let (pos_tx, pos_rx) = watch::channel((0.0, 0.0));
        ret.insert(i as u32, pos_rx);
        pos_txs.push(pos_tx);
    }

    tokio::spawn(async move {
        let mut curr: u32 = 0;

        loop {
            let input = fake_input_rx.recv();
            if let Err(_) = input {
                break;
            } else if let Ok(input) = input {
                match input {
                    RawMessage::Hover(pos) => { pos_txs[curr as usize].send(fix_pos(pos, window_size)).ok(); }
                    RawMessage::LeftClick(pos) => { msg_tx.send((curr, super::Message::Click(pos))).await.ok(); }
                    RawMessage::RightClick(pos) => { msg_tx.send((curr, super::Message::DoubleClick(pos))).await.ok(); }
                    RawMessage::MiddleClick => {
                        if curr == (count - 1) as u32 {
                            curr = 0;
                        } else {
                            curr += 1;
                        }
                    }
                }
            }
        }

    });

    return ret;
}


fn fix_pos(pos: PosCoord, window_size: (f32, f32)) -> PosCoord {
    let (width, height) = window_size;
    let (x, y) = pos;

    (x - width / 2.0, height / 2.0 - y)
}