use bytes::Buf;
use tokio::net::UdpSocket;
use tokio::runtime;
use std::io::Error;
use std::sync::Arc;
use std::thread::JoinHandle;
use super::{Status, CamCtnInfo, CamCtn};
use crate::net::server::proto::{Handler, Packet};

pub const DEFAULT_UDP_START_PORT: usize = 25565;
pub const MTU: usize = 10000000;

type Socket = Arc<UdpSocket>;
pub type SocketHandle = (Socket, JoinHandle<()>);



pub struct UdpCtn {
  info: CamCtnInfo,
  status: Status,
  socket: SocketHandle
}

impl UdpCtn {
  fn get_sock(&self) ->  Socket {
    let (sock, _) = self.socket;
    return sock
  }
}


impl CamCtn for UdpCtn {
  fn new<P: Packet>(info: CamCtnInfo,
                    handler: Handler<P>) -> Result<UdpCtn, Error> {
    let rt = runtime::Runtime::new()?;
    let send = rt.block_on(async {
      Arc::new(UdpSocket::bind(format!("{}:{}", info.addr, info.port)).await?)
    });
    let recv = send.clone();
    let mut status = Status::Unconnected;

    //this is dumb
    let handle = tokio::spawn(async move {
      let mut buf = [0; MTU];
      loop {
        let p: Option<P> = match recv.recv(buf).await {
          Ok(()) => Some(P::unmarshal(&mut buf)?),
          Err(_) => None,
        };
        match p {
          Some(packet) => {
            handler(packet)
          },
          None => {
            status = Status::Error;
            return
          }
        }
      }
    });
    Ok(UdpCtn {
      info,
      status,
      socket: (send, handle)
    })
  }

  fn close(&self) -> Result<(), Error> {
    let sock = self.get_sock();
    self.status = Status::Unconnected;
    Ok(())
  }


  fn send<P: Packet>(&self, p:P) -> Result<(), Error> {

    let mut buf = [0; MTU];
    p.marshal(&mut buf);
    let (sock, _) = self.socket;
    sock.send(buf)?;
    Ok(())
  }

  fn get_status(&self) -> Self {
    self.status
  }

  fn get_addr(&self) -> Self {
    self.info.addr
  }

  fn get_peer_addr(&self) -> Option<&'static str>{
    self.info.peer_addr
  }

}
