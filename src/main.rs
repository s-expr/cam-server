#![allow(unused_imports)]
#![allow(unused)]
extern crate nalgebra as na;

//mod obsmat;
mod tag_detector;
mod net;
mod util;
mod config;
mod ekf;
mod visualization;

use tag_detector::image::*;
use tag_detector::detector::*;
use apriltag::Detector;
use net::server;
use server::cam_ctn::udp::*;
use server::cam_ctn::{CamCtn, CamCtnInfo};
use server::proto::ts_custom::TagStreamPacket;
use server::proto::ts_custom::TagStreamHeader;
use server::proto::Packet;
use tokio::sync::mpsc;
use std::sync::Arc;
use na::{Vector2, Vector3, Matrix2, Matrix3, Matrix3x4, Matrix2x3};

use opencv::prelude::*;
use opencv::highgui;
use tokio::task::JoinHandle;



//mod visualization;


#[tokio::main]
async fn main() {
  let (tag_pos_tx, mut tag_pos_rx) = mpsc::unbounded_channel::<(TagID, Vector3<f64>)>();
  let ekf_tp = Arc::new(ekf::EKFThreadPool::new(tag_pos_tx, "./calibration.npy", &config::TAGS));
  let mut ctns: Vec<(UdpCtn<TagStreamPacket>, JoinHandle<()>)> = Vec::new();

  for i in 0..config::NUM_CAMERAS {
    let ekf_tp = ekf_tp.clone();
    let info: CamCtnInfo = CamCtnInfo {
      addr : config::ADDRESS,
      port : config::START_PORT + i,
      id : i,
    };

    let (ts_tx, mut ts_rx) = mpsc::channel::<TagStreamPacket>(config::MTU);
    let ctn = UdpCtn::<TagStreamPacket>::new(info, ts_tx).unwrap();

    // STAGE 1: Window Detection
    let cam_loop = tokio::spawn(async move {
      let mut wrap = Detwrapper{det:Detector::new("tagCustom48h12")};
      wrap.det.set_thread_number(8);
      wrap.det.set_decimation(1.0);
      wrap.det.set_sigma(2.0);

      println!("Finished building detector");
      loop {

        tokio::select!{
          p = ts_rx.recv() => {

            let mut packet = p.unwrap();
            let head = &mut packet.header;
            let w = head.width as usize;

            let img = &packet.data.as_aprilimg(w,w);

            println!("Received a window of size {}x{} px:{} py:{} fc:{} from camera {}",
                     head.width, head.width, head.px, head.py, head.ts, i);

            
            /*
            //visualizaition for debug
            let mat = Mat::from_slice_rows_cols(
            img.as_slice(),
            w, w
          );
            if let Err(_) = mat {
            continue;
          };


            highgui::imshow(window, &mat.unwrap());
            let key = highgui::wait_key(10);
             */

            
            let maybe_det = wrap.det.detect_one(img);
            if let Some((id, [center_x, center_y])) = maybe_det {
              println!("Detected tag id {}", id);
              //TODO: make less error prome with try_into(). 
              head.px += center_x as u16;
              head.py += center_y as u16;
              ekf_tp.send(id, i,  head.px as f64, head.py as f64, head.ts);
            }
          }
        }
      }
    });
    ctns.push((ctn, cam_loop));
  }


  let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
  
  //let (tag_tx, mut tag_rx) = mpsc::unbounded_channel::<(usize, (TagID, ekf::DetectionInfo))>();
  

  // STAGE 2: EKF


  // STAGE 3: Visualization
  visualization::visualize(&mut tag_pos_rx);
  /*
  loop {
    let (id, pt) = tag_pos_rx.recv().await.unwrap();
    let a = pt.x;
    let b = pt.y;
    let c = pt.z;
    print!("{id} x: {a} y: {b} z: {c}\n");
  }
  */
}


