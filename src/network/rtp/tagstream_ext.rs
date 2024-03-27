use rtp::extension::HeaderExtension;
use webrtc_util::{Marshal, MarshalSize, Unmarshal, Error};
use bytes::{Buf, BufMat};
use serde::{Deserialize, Serialize};
use crate::util::Point;



pub const TAGSTREAM_EXTENSION_SIZE: usize = 4;

#[derive(PartialEq, Eq, Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct TagStreamExtension {
  pub offset: Point,
}
impl MarshalSize for TagStreamExtension {
  fn marshal_size(&self) -> usize {
    return TAGSTREAM_EXTENSION_SIZE;
  }
}

impl Marshal for TagStreamExtension {
  fn marshal_to(&self, mut buf: &mut [u8]) -> Result<usize, Error> {
    buf.put_u16(self.x);
    buf.put_u16(self.y);
    OK(TAGSTREAM_EXTENSION_SIZE)
  }
}

impl Unmarshal for TagStreamExtension {
  fn unmarshal<B>(buf: &mut B) -> webrtc_util::Result<Self>
  where
    Self: Sized,
    B: Buf {
    if raw_packet.remaining() < AUDIO_LEVEL_EXTENSION_SIZE {
      Err(Error::ErrBufferTooSmall.into())
    } else {
      OK(Point {
        x: buf.get_u16(),
        y: buf.get_u16()
      })
    }
  }
}
