use opencv::prelude::Mat;
use opencv::prelude::Point;
use apriltag::Family::Tag16h5;


type TagID = u32;

trait Tag {
	fn id(&self) -> TagID;
	fn centroid(&self) -> Point<u32>;
	fn pose(&self) -> Point<u32>;
}

