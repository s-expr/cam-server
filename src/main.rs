#![allow(unused_imports)]
#![allow(unused)]
//pub mod tag_detector;
//mod obsmat;
pub mod net;
pub mod util;

use crate::net::server;
use server::cam_ctn::udp::*;
use server::cam_ctn::{CamCtn, CamCtnInfo};
use server::proto::ts_custom::TagStreamPacket;
use server::proto::Packet;
use tokio::sync::mpsc;
//mod visualization;
mod ekf;
const START_PORT: usize = 3333;
const MTU: usize = 100000;

fn new_ctn<P: Packet>(port: usize, tx: mpsc::Sender<P>) {
  let info: CamCtnInfo = CamCtnInfo {
    addr : "192.168.0.189",
    port : START_PORT,
    id : 0,
  };
  UdpCtn::<TagStreamPacket>::new(info, tx).unwrap();
}

#[tokio::main]
async fn main() {
  let (tx, mut rx) = mpsc::channel::<TagStreamPacket>(MTU);
  loop {
    tokio::select!{
      p = rx.recv() => {
        let packet = p.unwrap();
        println!("id: {}, width: {}",packet.header.camera_id, packet.header.width)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use tokio::runtime;
  #[test]
  pub fn test_udp_connection() {
    use crate::net::server::proto;
  }
}
