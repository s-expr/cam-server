use super::tag::{
	Tag,
	TagID
};

use opencv::prelude::Mat;
use apriltag::Family;

trait DetectorExt {
	fn detect(mat: &Mat, fam: Family) -> Vec<TagID>;
	//fn get_profile() -> u32; //get time it took to run
}

