extern crate kiss3d;
extern crate nalgebra as na;

use std::{f32::consts::PI, intrinsics::size_of};
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, Translation3};
use tokio::sync::mpsc;

const POINT_RADIUS: f32 = 0.1; // Increase the radius for larger point representation

pub struct TagPoint {
    pub id : u16,
    pub tr : Translation3,
}

#[tokio::spawn_vthread]
pub async fn spawn_vthread() {

    let (tx, mut rx) = mpsc::channel::<TagPoint>(10000);

    let jh = tokio::spawn(async move {

        // Create a window for rendering.
        let mut window = Window::new("Kiss3d: moving point");

        // Set the light to be in the front.
        window.set_light(Light::StickToCamera);
        window.set_background_color(0.8, 0.8, 0.8);

        // Time step for the animation.
        let mut time = 0.0f32;
        let time_step = 0.016f32; // Approximately 60Hz

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
            let recv_pt = rx.recv().await;

            let update_sphere = match(pt.id) {
                0 => sphere0,
                1 => sphere1,
                2 => sphere2,
            };

            // Draw axis
            window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(axis_length, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0));
            window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, axis_length, 0.0), &Point3::new(0.0, 1.0, 0.0));
            window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, axis_length), &Point3::new(0.0, 0.0, 1.0));
  
            // Set the new position for the sphere.
            update_sphere.set_local_translation(recv_pt.tr);

            // Increment the time for the animation.
            time += time_step;
        }
    });

    return (jh, tx);

}