mod rtp;
pub mod raw;

use bytes::BufMut;
use bytes::Buf;
use tokio::io::Error;

pub type Handler<P: Packet> = fn(P) -> ();


pub trait Packet: Send + Sized {
  fn marshal<B: BufMut>(&self, buf: &mut B);
  fn unmarshal<B: Buf>(buf: &mut B) -> Result<Self, ()>; 
}

