use std::path::{Path, PathBuf};
use std::fs;
use file_mmap::FileMmap;

fn main()  {
	let camera_width = 4056;
	let camera_height = 3040;
	let mut fm=FileMmap::new("./pycamera2_frame").unwrap();
	fm.set_len(camera_width * camera_height * 3 + 1).unwrap();
	println!("Hello");
	loop {
		
	}
}