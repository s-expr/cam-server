use crate::util;
use crate::net::server::proto::Packet;
use super::{Status, CamCtnInfo, CamCtn};

use tokio::sync;
use tokio::io;
use tokio::net::UdpSocket;
use std::io::Error;
use std::sync::Arc;
use std::thread::JoinHandle;

pub const DEFAULT_UDP_START_PORT: usize = 25565;
pub const MTU: usize = 10000000;

type SigChan<T> = sync::oneshot::Sender<T>;
struct SocketHandle<P: Packet> {
  kill_chan: SigChan<()>,
  send_chan: SigChan<P>,
  handle: JoinHandle<()>
};

pub struct UdpCtn<P: Packet> {
  info: CamCtnInfo,
  status: Status,
  socket_h: Option<SocketHandle<P>>
}

impl<P: Packet> UdpCtn<P> {
  //TODO: make this error tolerant
  fn send_kill_channel(&self, packet: P) -> Result<(), Error>  {
    let Some(SocketHandle::<P> {kill_chan, ..}) = self.socket_h;
    kill_chan.send(packet)?;
    Ok(())
  }

  //TODO: make this error tolerant
  fn send_packet_channel(&self, packet: P) {
  }
}

impl<P: Packet> CamCtn<P> for UdpCtn<P> {
  fn new(info: CamCtnInfo,
         packet_out: sync::mpsc::Sender<P> ) -> Result<UdpCtn<P> , Error> {

    let mut status = Status::Connected;
    let socket = util::sync(async {
      Arc::new(UdpSocket::bind(format!("{}:{}", info.addr, info.port)).await.unwrap())
    });
    let (close_tx , close_rx) = sync::oneshot::channel();
    let (packet_tx , packet_rx) = sync::oneshot::channel::<P>();
    let mut ctn = UdpCtn::<P> {
      info,
      status,
      socket_h: None
    };

    let handle = tokio::spawn(async {
      let mut buf = [0; MTU];
      loop {
        let err_or_p = match socket.recv(&mut buf).await {
          Ok(_) =>
            P::unmarshal(&mut buf),
          Err(io::ErrorKind::ConnectionAborted) => {
            ctn.status = Status::Error;
            return
          },
          e => {
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
    ctn.socket = Some((close_tx, handle));
    Ok(ctn)
  }

  fn close(&self) -> Result<(), Error> {
    let sock = self.get_sock();
    self.status = Status::Unconnected;
    Ok(())
  }


  fn send(&self, p:P) -> Result<(), Error> {

    let mut buf = [0; MTU];
    p.marshal(&mut buf);
    sock.send(buf)?;
    Ok(())
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
