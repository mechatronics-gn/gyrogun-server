use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use crate::client::SensorData;

pub enum RawMessage {
    #[deprecated]
    #[allow(dead_code)]
    Position(SensorData),
    Click(SensorData),
    DoubleClick(SensorData),
    SetIndex(u32),
}

impl RawMessage {
    pub async fn read(socket: &mut TcpStream) -> Option<RawMessage> {
        let mut buf = vec![0 as u8; 16];

        let n = socket.read_exact(&mut buf).await;

        if let Err(_) = n {
            return None;
        }

        let message_type = [buf[0], buf[1], buf[2], buf[3]];
        let message_type = i32::from_be_bytes(message_type);

        if message_type == 3 {
            let idx = [buf[4], buf[5], buf[6], buf[7]];
            let idx = u32::from_be_bytes(idx);

            return Some(RawMessage::SetIndex(idx));
        }

        let y = [buf[4], buf[5], buf[6], buf[7]];
        let y = f32::from_be_bytes(y);

        let p = [buf[8], buf[9], buf[10], buf[11]];
        let p = f32::from_be_bytes(p);

        let r = [buf[12], buf[13], buf[14], buf[15]];
        let r = f32::from_be_bytes(r);

        //println!("{message_type} {y} {p} {r}");

        if message_type == 0 {
            // return Some(RawMessage::Position((y, p, r)));
        } else if message_type == 1 {
            return Some(RawMessage::Click((y, p, r)));
        } else if message_type == 2 {
            return Some(RawMessage::DoubleClick((y, p, r)));
        }

        None
    }
}
