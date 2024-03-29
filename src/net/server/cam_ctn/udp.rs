use bytes::Buf;
use tokio::net::UdpSocket;
use super::{Status, CamCtnInfo, CamCtn};
use crate::net::proto::{Handler, Packet};
pub const DEFAULT_TCP_START_PORT: usize = 25565;

pub struct UdpCtn {
  info: CamCtnInfo,
  status: Status,
  handler: Handler
}

impl CamCtn for UdpCtn {

}
