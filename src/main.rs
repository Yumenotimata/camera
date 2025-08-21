use core::time;
use std::path::{Path, PathBuf};
use std::{fs, ptr};
use std::thread::sleep;
use file_mmap::FileMmap;
use image;
use minifb;

fn main()  {
	let camera_width = 4056;
	let camera_height = 3040;
	let mut fm=FileMmap::new("./pycamera2_frame").unwrap();
	fm.set_len(camera_width * camera_height * 3 + 1).unwrap();
	println!("Hello");
	
	let mut last_index = 0;
	loop {
		let p = fm.as_ptr();
		if last_index != unsafe{p.read_unaligned()} {
			last_index = unsafe{p.read_unaligned()};
			println!("changed!");
		}

		let mut img = image::RgbImage::new(camera_width as u32,camera_height as u32);
		unsafe {ptr::copy(p.add(1),img.as_mut_ptr(),(camera_width * camera_height * 3) as usize);}


		//img.save("mine.png").unwrap();

		sleep(time::Duration::from_millis(1000));
	}
}