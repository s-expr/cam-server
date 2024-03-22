use super::client::Connection;



pub struct ServerConfig {
	addr: &str,
	port: u16,
	transport: Protocol,
	connections: usize,
}

pub struct CameraServer {
	cfg: ServerConfig,
	protocol: Protocol,
	ctns: usize,
}

impl CameraServer {
	pub fn init(&self, cfg: ServerConfig);
	pub fn connect(&self, addr: &str);
	pub fn connections(&self) -> &Vec<Conection>;
	pub fn listen(&self);
	pub fn shutdown(&self);

	fn disconnect(&self, addr: &str);
	fn bind_addr(&self, addr: &str);
	fn port(&self, addr: u16);
}
