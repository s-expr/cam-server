use opencv::{
	highgui,
	prelude::*,
	imgproc::cvt_color,
	videoio,
	Result,
	Error,
};
use apriltag::{
	DetectorBuilder,
	families::Family,
	Image,
	image_buf::DEFAULT_ALIGNMENT_U8
};

// TODO?: may need to write an MatImageExt impl for Images
//        in the style of to_image_unsafe in mat2image.
//        from_image_buffer still does a full iterated copy? wtf


fn mat_to_img(mat: &Mat) -> Result<Image, Error> {
	mat.data();
	let w = mat.cols() as usize;
	let h = mat.cols() as usize;
	let buf = mat.data_bytes().unwrap();
	let mut image = Image::zeros_with_alignment(w, h, DEFAULT_ALIGNMENT_U8).unwrap();

	for (i, pixel) in buf.iter().enumerate() {
		image[(i%w, i/h)] = *pixel; 
	}
	Ok(image)
}

fn detect(mat: &Mat) -> Result<(), Error>{
	let builder = DetectorBuilder::new();
	let family: Family = "tag16h5".parse().unwrap();
	let mut det = builder.add_family_bits(family,1).build().unwrap();
	let img = mat_to_img(mat);

	for (index, d) in det.detect(&img?).into_iter().enumerate() {
			println!("  - detection {index}: {d:#?}");
	}
	Ok(())
}

#[test]
fn test_tag_recognition() -> Result<(), Error> {
let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
	loop {
		let mut img = Mat::default();
		let mut imgpleasegod = Mat::default();

	println!("detecting or whatever");

		cam.read(&mut img)?;
		cvt_color(&img, &mut imgpleasegod,opencv::imgproc::COLOR_BGR2GRAY, 1)?;
		detect(&imgpleasegod)?;

		if img.size()?.width > 0 {
			highgui::imshow(window, &img)?;
		}
		let key = highgui::wait_key(10)?;
		if key > 0 && key != 255 {
			break;
		}
	}
	Ok(())
}
