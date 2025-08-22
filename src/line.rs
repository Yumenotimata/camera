
use core::time;
use std::{u8, vec};
use file_mmap::FileMmap;

pub struct LineInfo{
	camera_width:u64,
	camera_height:u64,
	
	rect_width:u64,
	rect_height:u64,
	rect_offset:u64,

	last_index:u8,
	fm:FileMmap,
}

impl LineInfo {
	pub fn new(camera_width:u64,camera_height:u64,rect_height:u64,rect_width:u64,rect_offset:u64) -> Self{
		let mut fm=FileMmap::new("./pycamera2_frame").unwrap();
		fm.set_len(camera_width * camera_height * 3 + 1).unwrap();
		let last_index = 0;
		LineInfo{camera_width,camera_height,rect_width,rect_height,rect_offset,fm,last_index}
	}

	pub fn scan(&mut self,r_th:u8,g_th:u8,b_th:u8,rev:bool) -> Option<(u8,i32)>{
		let mut p = self.fm.as_ptr();
		// println!("{}", unsafe{p.read_unaligned()} );
		if self.last_index != unsafe{p.read_unaligned()} {
			self.last_index = unsafe{p.read_unaligned()};
			// println!("yet");

			unsafe{p = p.add((self.rect_offset * self.camera_width * 3 + 1) as usize)};

			let mut black_sense:Vec<u16> = vec![0;self.rect_width as usize];
			for i in 0..(self.camera_width * self.rect_height){
				let x = (i % self.camera_width) as usize / ((self.camera_width ) / (self.rect_width)) as usize ;
				let r = unsafe {*p.add(1)};
				let g = unsafe {*p.add(2)};
				let b = unsafe {*p.add(3)};
				p = unsafe {p.add(3)};
				if x < self.rect_width as usize && r <= r_th && g <= g_th && b <= b_th{
					black_sense[x] += 1;
				}
			}
		

			let distance = Self::find_closest_non_zero_distance(&black_sense);

			match distance {
				Some(d) => if rev {
					Some((0,-d))
				}else{
					Some((0,d))
				},
				None => None,
			}
			
			//display_image(image_width as usize,image_height as usize, image.as_ptr(),&mut window);
			//display_image(camera_width as usize,camera_height as usize, p,&mut window);
		}
		else {
			None
		}
	}

	fn find_closest_non_zero_distance(array: &Vec<u16>) -> Option<i32> {
		if array.is_empty() {
			return None;
		}

		let n = array.len() as i32;
		let center = (n - 1) / 2;
		
		let mut min_distance:i32 = -9999;

		for i in 0..n {
			if array[i as usize] != 0 {
				let distance = (i - center).abs();
				if distance < (min_distance.abs()) {
					min_distance = i - center;
				}
			}
		}

		if min_distance == -9999 {
			None
		} else {
			Some(min_distance)
		}
	}
}