mod transport;
mod proto;
mod cam_conn;

use super::client::Connection;




pub struct ServerConfig {
	transport: Protocol,
	connections: usize,
}

pub struct CameraServer {
	ctns: Vec<>
}

impl CameraServer {
	pub fn init(&self, cfg: ServerConfig);

	fn disconnect(&self, addr: &str);
	fn bind_addr(&self, addr: &str);
	fn port(&self, addr: u16);
}
