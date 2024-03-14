use opencv::highgui;
use opencv::prelude::*;
use opencv::videoio;
use opencv::Result;
use opencv::Error;
use apriltag::{DetectorBuilder};

#[test]
fn test_tag_recognition() -> Result<(), Error> {
	let detector = DetectorBuilder::new( );
	let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
	let var_name = videoio::VideoCapture::new(0, videoio::CAP_ANY);
	let mut cam = var_name?; // 0 is the default camera
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width > 0 {
			highgui::imshow(window, &frame)?;
		}
		let key = highgui::wait_key(10)?;
		if key > 0 && key != 255 {
			break;
		}
	}
	Ok(())
}
