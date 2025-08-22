use core::time;
use std::{u8, vec};
use std::thread::sleep;
use camera::line;
use file_mmap::FileMmap;
use minifb::{self, Window};

fn display_image(width:usize,height:usize,buffer:*const u8,window:&mut Window){

	let mut image_data:Vec<u32> = vec![0;width*height];
	for (index,c) in unsafe {std::slice::from_raw_parts(buffer, width * height * 3)}.iter().enumerate(){
		let pixel_index = index / 3;
		image_data[pixel_index] = (image_data[pixel_index] << 8) | *c as u32;
	}

	window.update_with_buffer(&image_data, width,height).unwrap();


}

fn main()  {
	let camera_width = 4056;
	let camera_height = 3040;

	let rect_height = 10;
	let rect_width = 10;
	let rect_offset = 0;

	let r_th = 50;
	let g_th = 50;
	let b_th = 50;

	let mut i = line::LineInfo::new(camera_width, camera_height, rect_height, rect_width, rect_offset)	;
	
	loop {
	

		let r = i.scan(r_th, g_th, b_th,false);

		match r {
			Some((_,d)) => println!("{}",d),
			None => {}
		}

		sleep(time::Duration::from_millis(20));
	}
}