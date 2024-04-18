extern crate nalgebra as na;
use na::{Vector2, Vector3, Matrix2, Matrix3, Matrix3x4, Matrix2x3};

// Assuming the camera_matrix is a 3x4 matrix and x is a 3x1 vector
fn motion_model(x: &Vector3<f64>, dt: u64) -> Vector3<f64> {
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
    most_recent_timestep: u64,
    dt: u64
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

    pub fn filter(&mut self, meas: Vector2<f64>, t: Matrix3x4<f64>, timestep: u64) {

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
