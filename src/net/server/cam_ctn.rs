pub mod serial;
pub mod udp;
pub mod tcp;

use bytes::Buf;
use std::io::Error;
use super::proto::{Packet, Handler};
use tokio::sync::mpsc::Sender;

#[derive(Copy, Clone)]
pub enum Status {
  Connected,
  Error,
  Unconnected,
}


pub struct CamCtnInfo {
  addr: &'static str,
  port: usize,
  peer_addr: Option<&'static str>,
  id: usize
}



pub trait CamCtn<P: Packet + Sized> {
  fn new(info: CamCtnInfo, packet_tx: Sender<P>)
         -> Result<Self, std::io::Error> where Self: Sized;
  //fn new_listener(handle: Handler) -> Self;

  fn close(self) -> Result<(), Error>;
  fn send(&mut self, p: P);
  fn get_status(&mut self) -> Status;
  fn get_addr(&self) -> &'static str;
  fn get_peer_addr(&self) -> Option<&'static str>;

  //fn bind(&self) -> Self;
  //fn new_port(&self, ) -> Self;
}
