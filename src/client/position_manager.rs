use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::watch;
use crate::client::init::InitData;
use crate::client::{PosCoord, screen_pos, SensorData, shooter_pos};

pub struct PositionManager {
    init_datas: HashMap<String, watch::Receiver<Option<InitData>>>,
    pos_txs: HashMap<String, watch::Sender<PosCoord>>,
}

impl PositionManager {
    pub fn new() -> Self {
        Self {
            init_datas: HashMap::new(),
            pos_txs: HashMap::new(),
        }
    }

    pub fn register(&mut self, addr: SocketAddr) -> (watch::Sender<Option<InitData>>, watch::Receiver<PosCoord>) {
        let (init_data_tx, init_data_rx) = watch::channel(None);
        let (pos_tx, pos_rx) = watch::channel((0., 0.));

        println!("Inserted init data rx for {addr}");
        self.init_datas.insert(addr.ip().to_string(), init_data_rx);
        self.pos_txs.insert(addr.ip().to_string(), pos_tx);

        (init_data_tx, pos_rx)
    }

    pub async fn run(&mut self, server_addr: &str) {
        let sock = UdpSocket::bind(server_addr).await.unwrap();
        println!("running udpsock at {}", sock.local_addr().unwrap());
        loop {
            let mut buf = [0 as u8; 12];
            if let Ok((_, client_addr)) = sock.recv_from(&mut buf).await {
                let Some(init_data) = self.init_datas.get(&client_addr.ip().to_string()).map(|x| *x.borrow()).flatten() else { continue; };

                let y = [buf[0], buf[1], buf[2], buf[3]];
                let y = f32::from_be_bytes(y);

                let p = [buf[4], buf[5], buf[6], buf[7]];
                let p = f32::from_be_bytes(p);

                let r = [buf[8], buf[9], buf[10], buf[11]];
                let r = f32::from_be_bytes(r);

                let sensor_data: SensorData = (y, p, r);

                let shooter_pos = shooter_pos(&init_data);
                let screen_pos = screen_pos(&init_data, sensor_data, shooter_pos);

                self.pos_txs.get(&client_addr.ip().to_string()).map(|x| x.send(screen_pos).unwrap());
            }
        }
    }
}