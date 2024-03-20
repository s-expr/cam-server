use super::tag::{
	Tag,
	TagID
};

use opencv::core::Mat;
use apriltag::Family;

trait DetectorExt {
	fn detect(&Mat, Family) -> Vec<TagID>;
	//fn get_profile() -> u32; //get time it took to run
}

