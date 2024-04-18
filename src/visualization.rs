extern crate kiss3d;
extern crate nalgebra as na;

use std::{borrow::Borrow, f32::consts::PI};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, Translation3};
use tokio::{io::Join, net::unix::pipe::Sender, sync::mpsc, task::JoinHandle};

const POINT_RADIUS: f32 = 0.1; // Increase the radius for larger point representation

pub struct TagPoint {
    pub id : u16,
    pub tr : Translation3<f32>,
}

pub fn spawn_vthread() -> (JoinHandle<()>, mpsc::Sender<TagPoint>) {

    let (tx, mut rx) = mpsc::channel::<TagPoint>(10000);

    let jh = tokio::spawn(async move {
        
        let window = Window::new("Kiss3d: moving point");

        // Set the light to be in the front.
        window.set_light(Light::StickToCamera);
        window.set_background_color(0.8, 0.8, 0.8);

        // Create a sphere that will represent the point.
        let mut sphere0 = window.add_sphere(POINT_RADIUS);
        let mut sphere1 = window.add_sphere(POINT_RADIUS);
        let mut sphere2 = window.add_sphere(POINT_RADIUS);

        // Set the color of the sphere (optional, default is white).
        sphere0.set_color(1.0, 0.0, 0.0); // Red
        sphere1.set_color(0.0, 1.0, 0.0); // Green
        sphere2.set_color(0.0, 0.0, 1.0); // Blue

        // Axis length.
        let axis_length = 4.0;

        // Main loop.
        while window.render() {
            let recv_pt = rx.recv().await.unwrap();

            let update_sphere = match(recv_pt.id) {
                0 => &mut sphere0,
                1 => &mut sphere1,
                2 => &mut sphere2,
                _ => continue,
            };

            // Draw axis
            window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(axis_length, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0));
            window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, axis_length, 0.0), &Point3::new(0.0, 1.0, 0.0));
            window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, axis_length), &Point3::new(0.0, 0.0, 1.0));
  
            // Set the new position for the sphere.
            update_sphere.set_local_translation(recv_pt.tr);
        }
    });

    return (jh, tx);

}