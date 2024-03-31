use bytes::{Buf, BufMut};
use tokio::io::Error;
use super::Packet;

impl<B: Buf> Packet for B {
  fn unmarshal(buf: &mut B) -> Result<Self, Error> {
    Ok(buf.clone())
  }

  fn marshal(&self, buf: &mut B) -> Result<(), Error> {
    Ok(())
  }
}

