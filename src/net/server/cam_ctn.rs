mod serial;
mod udp;
mod tcp;


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
  remote_addr: Option<&'static str>,
}


//we love an existential type
trait CamCtn {
  fn new_listener(&self, handle: Handler) -> Self;
  fn new(&self, handle: Handler, info: CamCtnInfo) -> Self;
  fn status(&self) -> Status;
  fn close(&self);
  fn send(&self, p: Packet);
  fn get_addr(&self) -> &'static str;
  fn get_remote_addr(&self) -> Option<&'static str>;
  fn new_port(&self, ) -> Self;
  fn bind(&self) -> Self;

  fn set_handler(&self, handle: Handler);
}
