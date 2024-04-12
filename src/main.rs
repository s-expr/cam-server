#![allow(unused_imports)]
#![allow(unused)]
pub mod tag_detector;
//mod obsmat;
pub mod net;
pub mod util;


#[tokio::main]
async fn main() {
  println!("Hello, world!");
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
