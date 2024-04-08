use bytes::{Buf, BufMut};
use super::Packet;

impl Packet for &[u8] {
  fn unmarshal<B: Buf>(buf: &mut B) -> Result<Self, ()> {
    Ok(buf.chunk())
  }

  fn marshal<B: BufMut>(&self, buf: &mut B)  {
    buf.put_slice(self)
  }
}

