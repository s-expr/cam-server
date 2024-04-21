use bytes::{Buf, BufMut};
use super::Packet;

pub struct TagStreamHeader {
  pub cam_id : u8,
  pub px : u16,
  pub py : u16,
  pub width : u16,
  pub ts: u32,
}

pub struct TagStreamPacket {
  pub header: TagStreamHeader,
  pub data: Vec<u8>,
}

impl Packet for TagStreamPacket {
  fn unmarshal<B: Buf>(buf: & mut B) -> Result<Self, ()> {
    let header = TagStreamHeader {
        cam_id : buf.get_u8(),
        px : buf.get_u16(),
        py : buf.get_u16(),
        width : buf.get_u16(),
        ts: buf.get_u32(),
      };
    buf.advance(1);
    Ok(TagStreamPacket {
      header: header,
      data: buf.chunk().to_vec()
    })
  }

  fn marshal<B: BufMut>(&self, buf: &mut B)  {
    buf.put_u8(self.header.cam_id);
    buf.put_u16(self.header.px);
    buf.put_u16(self.header.py);
    buf.put_u16(self.header.width);
    buf.put_u32(self.header.ts);
    buf.put_slice(&self.data)
  }
}

