use core::time;
use std::{vec};
use std::thread::sleep;
use file_mmap::FileMmap;
use image;
use minifb::{self, Window};

fn display_image(width:usize,height:usize,buffer:*const u8,window:&mut Window){

	//let mut img = image::RgbImage::new(width as u32,height as u32);
	//unsafe {ptr::copy(buffer,img.as_mut_ptr(),width * height * 3);}
	//img.save("mine.png").unwrap();
	let mut image_data:Vec<u32> = vec::Vec::with_capacity(width*height);
	for (index,c) in unsafe {std::slice::from_raw_parts(buffer, width * height * 3)}.iter().enumerate(){
		let pixel_index = index / 3;
		image_data[pixel_index] = (image_data[pixel_index] << 8) | *c as u32;
	}

	window.update_with_buffer(&image_data, width,height).unwrap();


}

fn main()  {
	let camera_width = 4056;
	let camera_height = 3040;
	let mut fm=FileMmap::new("./pycamera2_frame").unwrap();
	fm.set_len(camera_width * camera_height * 3 + 1).unwrap();
	println!("Hello");


	let mut window = minifb::Window::new(
	"Minifb Image Viewer",
	camera_width as usize,
	camera_height as usize,
	minifb::WindowOptions::default(),
	).unwrap();
	
	let mut last_index = 0;
	loop {
		let p = fm.as_ptr();
		if last_index != unsafe{p.read_unaligned()} {
			last_index = unsafe{p.read_unaligned()};
			println!("changed!");
			display_image(camera_width as usize, camera_height as usize, unsafe{p.add(1)},&mut window);
		}

		sleep(time::Duration::from_millis(1000));
	}
}