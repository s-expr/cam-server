pub mod proto;
pub mod cam_ctn;

use cam_ctn::{CamCtn, Status};
use proto::{Packet, Handler};

pub struct CameraServer {
	ctns: Vec<CamCtn>,
  listeners: Vec<CamCtn>,
  handler: Handler<Packet>,
}

impl CameraServer {
	pub fn new_listener(&self, ctn: dyn CamCtn) {
  }

	fn on_connect(&self, ctn: dyn CamCtn) {
  }

	fn status(&self) -> Vec<Status> {
    self.ctns.map(|ctn| ctn.status())
  }

  fn handle_conn(&self, ctn: dyn CamCtn) {

  }

	fn disconnect(&self, id: usize) {

  }


	fn shutdown(&self ) {

  }


}
