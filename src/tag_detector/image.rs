use apriltag::Image;
use apriltag::image_buf::DEFAULT_ALIGNMENT_U8;
use opencv::prelude::Mat;

trait MatExt {
	fn as_aprilimg(&self) -> Image;
}

impl MatExt for Mat {
	fn as_aprilimg(&self) -> Image {
		let w = self.cols() as usize;
		let h = self.rows() as usize;
		let buf = self.data_bytes().unwrap();
		let mut image = Image::zeros_with_alignment(w, h, DEFAULT_ALIGNMENT_U8).unwrap();
		for (i, pixel) in buf.iter().enumerate() {
			image[(i%w, i/h)] = *pixel; 
		}
		image
	}
}
