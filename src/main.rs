use minifb::{CursorStyle, Window, Key, Scale, WindowOptions, MouseMode};

use rand::Rng;

const AREA_SIZE: usize = 15;

struct vector {
	x: i32, 
	y: i32 
}

struct snake {
	name: String, 
	size : u32, 
	position: vector,
	up: bool,
	right: bool,
	down: bool,
	left: bool
}

impl snake {
	fn new(_name: String) -> Self{
		snake {name: _name, size: 1, position: vector {x: AREA_SIZE as i32 / 2, y: AREA_SIZE as i32 / 2}, up: true, right: false, down: false, left: false}
	}	

	fn logic(snake: &mut snake, area: &mut Vec<u32>, window: &mut Window){
		if window.is_key_down(Key::W) && !snake.down {
			snake.up = true;
			snake.right = false;
			snake.down = false;
			snake.left = false;
		}
		else if window.is_key_down(Key::D) && !snake.left {
			snake.up = false;
			snake.right = true;
			snake.down = false;
			snake.left = false;
		}
		else if window.is_key_down(Key::S) && !snake.up {
			snake.up = false;
			snake.right = false;
			snake.down = true;
			snake.left = false;
		}
		else if window.is_key_down(Key::A) && !snake.right{
			snake.up = false;
			snake.right = false;
			snake.down = false;
			snake.left = true;
		}

		if snake.up {
			snake.position.y -= 1;
		}
		if snake.position.y < 0 {
			snake.position.y = AREA_SIZE as i32 - 1;
		}
		if snake.right {
			snake.position.x += 1;			
		}
		if snake.position.x == AREA_SIZE as i32{
				snake.position.x = 0;
		}
		if snake.down {
			snake.position.y += 1;
		}
		if snake.position.y == AREA_SIZE as i32 {
				snake.position.y = 0;
		}
		if snake.left {
			snake.position.x -= 1;
		}
		if snake.position.x < 0 {
			snake.position.x = AREA_SIZE as i32 - 1;
		}

		if area[snake.position.y as usize * AREA_SIZE + snake.position.x as usize] == 0 {
			snake.size += 1;
			let mut counter = 0;
			let mut index = rand::thread_rng().gen_range(0, AREA_SIZE * AREA_SIZE) as usize;
			while area[index] != 1 {
				index = rand::thread_rng().gen_range(0, AREA_SIZE * AREA_SIZE) as usize;
			}
			area[index] = 0;
		}

		if area[snake.position.y as usize * AREA_SIZE + snake.position.x as usize] > 1{
			panic!("game over");
		}

		for i in 0..area.len() {
			if (snake.position.y as usize * AREA_SIZE) + snake.position.x as usize == i{
        		area[i] = snake.size+1;
        	}
        	else {
        		if area[i] > 1{
        			area[i] = area[i] -1;
        		}
        	}
		}
	}
}

fn main() {
	let mut snake = snake::new("Joe".to_string());
	let mut area: Vec<u32> = vec![1; AREA_SIZE * AREA_SIZE];
	area[0] = 0;
	let mut buffer: Vec<u32> = vec![1; AREA_SIZE * AREA_SIZE];
	let mut window = Window::new(&snake.name.to_string(),
                                 AREA_SIZE,
                                 AREA_SIZE,
                                 WindowOptions {
                                     resize: false,
                                     scale: Scale::X32,
                                     ..WindowOptions::default()
                                 })
	.expect("Unable to Open Window");

	while window.is_open() && !window.is_key_down(minifb::Key::Escape){
		for i in 0..area.len() {
			buffer[i] = match area[i] {
				0 => 0xFF00E436, // apple
				1 => 0xFFFFF1E8, // nothing
				_ => 0xFFFF004D, // snake "Joe" :^)
			};
		}
		snake::logic(&mut snake, &mut area, &mut window);
		window.update_with_buffer(&buffer).unwrap();
		println!("x: {}, y: {}", &snake.position.x , &snake.position.y);
		std::thread::sleep(std::time::Duration::from_millis(100));
	}
}