#![allow(unused_imports)]
#![allow(unused)]
pub mod tag_detector;
//mod obsmat;
pub mod net;
pub mod util;
mod visualization;
mod ekf;

extern crate nalgebra as na;

  use visualization::{spawn_vthread, TagPoint};
use na::{Translation, Translation3};

#[tokio::main]
async fn main() {
  println!("Hello, world!");
  let (jh, tx) = spawn_vthread();

  // Time step for the animation.
  let mut time = 0.0f32;
  let time_step = 0.016f32; // Approximately 60Hz

  loop {
    let tp = TagPoint{id: 0, tr: Translation3::new(time.sin(), time.cos(), (time * 0.5).sin())};
    tx.send(tp);
    time += time_step;
  }
}

#[cfg(test)]
mod tests {
  use tokio::runtime;
  #[test]
  pub fn test_udp_connection() {
    use crate::net::server::cam_ctn;
    use crate::net::server::proto;
  }
}
