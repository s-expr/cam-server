use opencv::prelude::Mat;
use opencv::core::Point;
use apriltag::Family::Tag16h5;


pub type TagID = usize;

pub trait Tag<T> {
	fn id(&self) -> TagID;
	fn centroid(&self) -> Point;
	fn pose(&self) -> Point;
}

