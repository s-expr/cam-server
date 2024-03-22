
trait Connection {
  pub fn query(&self) -> ConnectionInfo; 
  pub fn disconnect(&self);
  pub fn remote_addr(&self) -> &str; 
}

