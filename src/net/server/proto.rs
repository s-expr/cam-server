//mod rtp;
pub mod raw;

use bytes::Buf;
use tokio::io::Error;

pub type Handler<P: Packet> = fn(P) -> ();

pub trait Packet {
  fn marshal<E, B: Buf>(&self, buf: &mut B) -> Result<(), E>;
  fn unmarshal<E, B: Buf, P: Sized>(buf: &mut B) -> Result<P, E>; 
}
