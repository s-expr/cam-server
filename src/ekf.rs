extern crate nalgebra as na;
use na::{Vector2, Vector3, Matrix2, Matrix3, Matrix3x4, Matrix2x3};
use std::collections::HashMap;
use tokio::task::JoinHandle;
use tokio::sync::{mpsc, mpsc::{UnboundedSender, UnboundedReceiver}};
use npyz;
use crate::config;

use crate::tag_detector::detector::TagID;

// Assuming the camera_matrix is a 3x4 matrix and x is a 3x1 vector
fn motion_model(x: &Vector3<f64>, dt: u32) -> Vector3<f64> {
  x + Vector3::new(
    dt as f64 * 0.1, 
    dt as f64 * 0.1, 
    dt as f64 * 0.1
  )
}

fn motion_jacobian(x: &Vector3<f64>) -> Matrix3<f64> {
  Matrix3::identity()
}

fn measurement_model(x: &Vector3<f64>, camera_matrix: &Matrix3x4<f64>) -> Vector2<f64> {
  camera_matrix.fixed_slice::<2,3>(0, 0) * x + camera_matrix.fixed_slice::<2,1>(0, 3)
}
fn measurement_jacobian(camera_matrix: &Matrix3x4<f64>) -> Matrix2x3<f64> {
  Matrix2x3::from(camera_matrix.fixed_slice::<2,3>(0, 0))
}

pub struct EKF {
  r: Matrix2<f64>,
  q: Matrix3<f64>,
  pub x: Vector3<f64>,
  cov: Matrix3<f64>,
  cov_init: Matrix3<f64>,
  most_recent_timestep: u32,
  dt: u32
}

impl EKF {
  pub fn new(meas_cov: Matrix2<f64>, proc_cov: Matrix3<f64>, 
             x_init: Vector3<f64>, cov_init: Matrix3<f64>) -> EKF {
    EKF {
      r: meas_cov,
      q: proc_cov,
      x: x_init,
      cov: cov_init,
      cov_init: cov_init,
      most_recent_timestep: 0,
      dt: 0
    }
  }

  pub fn filter(&mut self, meas: Vector2<f64>, t: Matrix3x4<f64>, timestep: u32) {

    if timestep < self.most_recent_timestep {
      return;
    } else if timestep > self.most_recent_timestep {
      self.dt = timestep - self.most_recent_timestep;
      self.most_recent_timestep = timestep;
      self.cov = self.cov_init;

      let f = motion_jacobian(&self.x);
      self.x = motion_model(&self.x, self.dt);
      self.cov = &f * &self.cov * f.transpose() + &self.q;
    }

    let h = measurement_jacobian(&t);
    let y = meas - measurement_model(&self.x, &t);

    let s = &h * &self.cov * h.transpose() + &self.r;
    let k = &self.cov * (h.transpose() * s.try_inverse().unwrap());

    self.x += &k * y;
    self.cov = (Matrix3::identity() - &k * &h) * &self.cov;
  }

}

pub type FilterArgs = (Vector2<f64>, u32);
type Snd<T> = UnboundedSender<T>;
type Recv<T> = UnboundedReceiver<T>;

pub struct EKFThreadPool {
  threads: HashMap<TagID, (Snd<(Matrix3x4<f64>, FilterArgs)>, JoinHandle<()>)>,
  calibration: HashMap<usize, Matrix3x4<f64>>,
  pub rx: Recv<(usize, (TagID, FilterArgs))>,
  tx: Snd<(TagID, Vector3<f64>)>,
}

impl EKFThreadPool {
  pub fn new(tp_tx:Snd<(TagID, Vector3<f64>)>,
             mut tp_rx:Recv<(usize, (TagID, FilterArgs))>,
             calibration: &str,
             ids: &[TagID]) -> EKFThreadPool {
    // create an ekf for each tag and parallelize it into a thread
    let cal_file = std::fs::read(calibration).unwrap();
    let cal_data = npyz::NpyFile::new(&cal_file[..]).unwrap().into_vec::<f64>().unwrap();
    let mut calibration: HashMap<usize, Matrix3x4<f64>> = HashMap::new();
    for i in 0..config::NUM_CAMERAS {
    //make compatible with more than 2 cameras
      calibration.insert(i,Matrix3x4::<f64>::from_row_slice(&cal_data[i*12..(i*12+12)]));
    }
    let threads = ids.iter().fold(HashMap::new(), |mut tds, id| {
      let (td_tx, mut td_rx) = mpsc::unbounded_channel::<(Matrix3x4<f64>,FilterArgs)>();
      let ekf = EKF::new(Matrix2::from_element(1e-3),
                         Matrix3::from_element(1e-4),
                         Vector3::zeros(),
                         Matrix3::zeros());
        //  proc_cov?
      tds.insert(*id, (td_tx, Self::new_proxy(ekf, tp_tx.clone(), td_rx, *id))); 
      return tds
    });

    EKFThreadPool {
      threads, 
      calibration,
      rx: tp_rx,
      tx: tp_tx,
    }
  }

  // Wraps an ekf into a thread and allows them
  // all to output to the same output
  fn new_proxy(mut ekf: EKF, tx: Snd<(TagID, Vector3<f64>)>,
                mut rx: Recv<(Matrix3x4<f64>, FilterArgs)>, id: usize) -> JoinHandle<()>{
    tokio::spawn(async move {
      let (t, (meas, timestep)) = rx.recv().await.unwrap();
      ekf.filter(meas,t, timestep);
      tx.send((id, ekf.x));
    })
  }

  // This is just for encapuslation. If speed
  // is an issuse we can throw away the proxy
  // channels and send directly to each ekf
  // thread.
  pub fn start_loop(self) -> JoinHandle<()> {
    tokio::spawn(async move {
      let mut rx = self.rx;
      loop {
        // doing this in a select for modularity
        // if we need later
        
        tokio::select!{
          arg = rx.recv() => {
            let (id,(tag, args)) = arg.unwrap();
            self.threads[&tag].0.send((self.calibration[&id], args));
          }
        }
      }
    })
  }

}


                          


