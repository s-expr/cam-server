mod table;

// Assume a fixed number of cameras and markers.
// Should be trivial to write the ability to dynamically
// Extend or shrink the size of the observation matrix.
// Based on new or removed cameras

struct Obn<T, Rows> {
	data: [T; Rows]
}

struct ObnTable<T, Rows> {
	id: usize,
	data: Vec<Obsn<T, Rows>>,
}

pub trait ObsnTable<Obn> {
	fn insert(&self, obn: Obn);
	fn delete(&self, idx);
	fn read(&self) -> Obn;
	fn peek(&self) -> &Obn;
}

struct ObnMat<T> {
	numu16
}

impl ObnMat {


}






