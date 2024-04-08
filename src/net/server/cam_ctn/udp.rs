use crate::util;
use crate::net::server::proto::Packet;
use super::{Status, CamCtnInfo, CamCtn};

use tokio::sync::oneshot;
use tokio::sync;
use tokio::net::UdpSocket;
use std::sync::Arc;
use std::io::Error;

pub const DEFAULT_UDP_START_PORT: usize = 25565;
pub const MTU: usize = 10000000;

type Sender<T> = sync::oneshot::Sender<T>;

struct NetThreadHandle<P: Packet> {
  close: Sender<()>,
  packet: Sender<P>,
  handle: tokio::task::JoinHandle<()>
}

pub struct UdpCtn<P: Packet> {
  info: CamCtnInfo,
  status: Status,
  socket_h: Option<NetThreadHandle<P>>
}

impl<P: Packet> NetThreadHandle<P> {
  //TODO: make this error tolerant
  fn req_close(&self) -> Result<(), tokio::task::JoinError> {
    self.close.send(());
    util::sync(async {
      self.handle.await
    })
  }

  //TODO: make this error tolerant
  fn send_packet(&self, packet: P) -> Result<(), P> {
    self.packet.send(packet)
  }
}

impl<P: Packet> CamCtn<P> for UdpCtn<P> {
  fn new(info: CamCtnInfo, packet_out: sync::mpsc::Sender<P> )
         -> Result<UdpCtn<P> , std::io::Error> {
    let mut status = Status::Connected;
    let socket = util::sync(async {
      Arc::new(UdpSocket::bind(format!(
        "{}:{}",
        info.addr,
        info.port
      )).await.unwrap())
    });
    let (close_tx , close_rx) = oneshot::channel();
    let (packet_tx , packet_rx) = oneshot::channel::<P>();
    let mut ctn = UdpCtn::<P> {
      info,
      status,
      socket_h: None
    };

    let handle = tokio::spawn(async {
      let mut buf = bytes::BytesMut::with_capacity(MTU);
      tokio::select! {
        p = socket.recv(&mut buf) => {
          println!("packet");
        }
      }
      loop {
        let err_or_p = match socket.recv(&mut buf).await {
          Ok(_) => {
            P::unmarshal(&mut buf)
          },
          e => {
            ctn.status = Status::Error;
            println!("unable to receive packet");
            continue 
          },
        };
        
        match err_or_p {
          Ok(packet) => {
            packet_out.send(packet);
          },
          e => {
            println!("Invalid packet format");
            continue
          }
        }
      }
    });
    ctn.socket_h = Some(NetThreadHandle {
      close: close_tx,
      packet: packet_tx,
      handle
    });
    Ok(ctn)
  }

  fn close(&self) -> Result<(), Error> {
    self.status = Status::Unconnected;
    Ok(())
  }


  fn send(&self, p:P) {
  }

  fn get_status(&self) -> Status {
    self.status
  }

  fn get_addr(&self) -> &'static str {
    self.info.addr
  }

  fn get_peer_addr(&self) -> Option<&'static str>{
    self.info.peer_addr
  }
}
