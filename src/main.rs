use core::time;
use std::{u8, vec};
use std::thread::sleep;
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
	let image_width = 100;
	let image_height = 100;

	let ipix_per_cpix_w = camera_width / image_width;
	let ipix_per_cpix_h = camera_height / image_height;

	let mut fm=FileMmap::new("./pycamera2_frame").unwrap();
	fm.set_len(camera_width * camera_height * 3 + 1).unwrap();
	println!("Hello");


	let mut window = minifb::Window::new(
	"Minifb Image Viewer",
	image_width as usize,
	image_height as usize,
	minifb::WindowOptions::default(),
	).unwrap();
	
	
	let mut last_index = 0;
	loop {
		let mut p = fm.as_ptr();
		if last_index != unsafe{p.read_unaligned()} {
			last_index = unsafe{p.read_unaligned()};
			println!("changed!");

			let mut image_sum:Vec<u64> = vec![0;(image_width*image_height * 3 )as usize];
			for i in 0..(camera_width*camera_height*3)as u64{
				let color_index = i % 3;
				let pixel_index
				let x = (i % camera_width) / ipix_per_cpix_w as u64;
				let y = (i / camera_width) / ipix_per_cpix_h as u64;
				p = unsafe{p.add(1)};
				image_sum[(x + image_width as u64 * y) as usize] += unsafe{*p} as u64;
				println!("{:?},{:?}",x,y);
			}


			let mut image:Vec<u8> = vec![0;(image_width*image_height * 3) as usize];
			for (i,e) in image_sum.iter().enumerate(){
				image[i] = (*e / (ipix_per_cpix_w * ipix_per_cpix_h)) as u8;
				//println!("{:?}",*e);

			}

			//println!("{:?}",image);

			display_image(image_width as usize,image_height as usize, image.as_ptr(),&mut window);
		}

		sleep(time::Duration::from_millis(1000));
	}
}