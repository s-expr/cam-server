use bytes::{Buf, BufMut};
use super::Packet;

pub struct TagStreamHeader {
  pub camera_id : u8,
  pub px : u16,
  pub py : u16,
  pub width : u16,
  pub ts_useconds: f64,
  pub ts_seconds: f64,
}

pub struct TagStreamPacket {
  pub header: TagStreamHeader,
  pub data: Vec<u8>,
}


impl Packet for TagStreamPacket {
  fn unmarshal<B: Buf>(buf: & mut B) -> Result<Self, ()> {
    let header = TagStreamHeader {
        camera_id : buf.get_u8(),
        px : buf.get_u16(),
        py : buf.get_u16(),
        width : buf.get_u16(),
        ts_useconds : buf.get_f64(),
        ts_seconds : buf.get_f64(),
      };
    Ok(TagStreamPacket {
      header: header,
      data: buf.chunk().to_vec()
    })
  }

  fn marshal<B: BufMut>(&self, buf: &mut B)  {
    buf.put_u8(self.header.camera_id);
    buf.put_u16(self.header.px);
    buf.put_u16(self.header.py);
    buf.put_u16(self.header.width);
    buf.put_f64(self.header.ts_useconds);
    buf.put_f64(self.header.ts_seconds);
    buf.put_slice(&self.data)
  }
}

