extern crate kiss3d;
extern crate nalgebra as na;

use std::f32::consts::PI;
use std::collections::HashMap;
use kiss3d::{light::Light, scene::SceneNode};
use kiss3d::window::Window;
use na::{Point3, Translation3};
use crate::{
  tag_detector::detector::TagID,
  config,
};

use tokio::sync::{
  mpsc,
  mpsc::UnboundedReceiver,
};


type TagPoint = (TagID, na::Vector3<f64>);
const POINT_RADIUS: f32 = 0.1; // Increase the radius for larger point representation

    
pub async fn visualize(points_rx: &mut UnboundedReceiver<TagPoint>) {

  loop {

    // Create a window for rendering.
    let mut window = Window::new("Kiss3d: moving point");

    // Set the light to be in the front.
    window.set_light(Light::StickToCamera);
    window.set_background_color(0.8, 0.8, 0.8);

    // Time step for the animation.
    let mut time = 0.0f32;
    let time_step = 0.016f32; // Approximately 60Hz

    // Create a spheres that will represent the point.
    let spheres: HashMap<TagID, SceneNode> = HashMap::new();
    for i in config::TAGS {
      spheres[&i] = window.add_sphere(POINT_RADIUS);
    }

    // Set the color of the sphere (optional, default is white).
    for (_, sphere) in spheres {
      sphere.set_color(1.0, 0.0, 0.0); // Red
    }

    // Axis length.
    let axis_length = 4.0;

    // Main loop.
    while window.render() {
      let (id, pos) = points_rx.recv().await.unwrap();

      for (_, sphere) in spheres {
        sphere.set_color(1.0, 0.0, 0.0); // Red
      }

      // Draw axis
      window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(axis_length, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0));
      window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, axis_length, 0.0), &Point3::new(0.0, 1.0, 0.0));
      window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, axis_length), &Point3::new(0.0, 0.0, 1.0));
      
      // Set the new position for the sphere.
      spheres[&id].set_local_translation(recv_pt.tr);

      // Increment the time for the animation.
      time += time_step;
    }
  }
}
