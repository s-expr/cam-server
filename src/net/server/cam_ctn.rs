pub mod serial;
pub mod udp;
pub mod tcp;

use bytes::Buf;
use super::proto::{Packet, Handler};

pub enum Status {
  Connected,
  Error,
  Unconnected,
}

pub struct CamCtnInfo {
  addr: &'static str,
  port: usize,
  peer_addr: Option<&'static str>,
}


//we love an existential type
pub trait CamCtn {
  fn new<P: Packet>(info: CamCtnInfo, handle: Handler<P>) -> Self;
  //fn new_listener(handle: Handler) -> Self;

  fn close<E>(&self) -> Result<(), E>;
  fn send<P: Packet>(&self, p: P);

  fn get_status(&self) -> Status;
  fn get_addr(&self) -> &'static str;
  fn get_peer_addr(&self) -> Option<&'static str>;

  //fn bind(&self) -> Self;
  //fn set_handler(&self, handle: Handler);
  //fn new_port(&self, ) -> Self;
}
