pub mod proto;
pub mod cam_ctn;

use cam_ctn::{CamCtn, Status};
use proto::Packet;

type Ctn<P> = Box< dyn CamCtn<P>>;

pub struct CameraServer<P: Packet> {
	ctns: Vec<Ctn<P>>,
  listeners: Vec<Ctn<P>>,

}

impl<P: Packet> CameraServer<P> {
	pub fn new_listener(&self, ctn: Ctn<P>) {
  }

	fn on_connect(&self, ctn: Ctn<P>) {
  }

	fn status(&self) -> Vec<Status> {
    self.ctns.into_iter().map(|ctn| ctn.get_status()).collect()
  }

  fn handle_conn(&self, ctn: Ctn<P>) {

  }

	fn disconnect(&self, id: usize) {

  }


	fn shutdown(&self ) {

  }


}
