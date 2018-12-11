use minifb::{Window, Key, Scale, WindowOptions};

use rand::Rng;

const AREA_SIZE: usize = 20;

struct Vector {
	x: i32, 
	y: i32 
}

struct Snake {
	name: String, 
	size : u32, 
	position: Vector,
	direction: u8, // 0 - up, 1 - right, 2 - down, 3 - left
	score: u32,
	is_alive: bool
}

impl Snake {
	fn new(_name: String) -> Self{
		Snake {name: _name, size: 1, position: Vector {x: AREA_SIZE as i32 / 2, y: AREA_SIZE as i32 / 2}, direction: 0, score: 0, is_alive: true}
	}	

	fn logic(snake: &mut Snake, area: &mut Vec<u32>, window: &mut Window) {
		if window.is_key_down(Key::W) && snake.direction != 2 {
			snake.direction = 0;
		}
		else if window.is_key_down(Key::D) && snake.direction != 3 {
			snake.direction = 1;
		}
		else if window.is_key_down(Key::S) && snake.direction != 0 {
			snake.direction = 2;
		}
		else if window.is_key_down(Key::A) && snake.direction != 1 {
			snake.direction = 3;
		}

		snake.position.x = match snake.direction {
			1 => snake.position.x + 1,
			3 => snake.position.x - 1,
			_ => snake.position.x,
		};

		if snake.position.x < 0 {
			snake.position.x = AREA_SIZE as i32 - 1;
		}

		if snake.position.x == AREA_SIZE as i32 {
			snake.position.x = 0;
		}

		snake.position.y = match snake.direction {
			0 => snake.position.y - 1,
			2 => snake.position.y + 1,
			_ => snake.position.y,
		};

		if snake.position.y < 0 {
			snake.position.y = AREA_SIZE as i32 - 1;
		}

		if snake.position.y == AREA_SIZE as i32 {
			snake.position.y = 0;
		}

		if area[snake.position.y as usize * AREA_SIZE + snake.position.x as usize] == 0 {
			snake.size += 1;
			snake.score +=1;
    		window.set_title(&format!("Score: {}", &snake.score));
			let mut index = rand::thread_rng().gen_range(0, AREA_SIZE * AREA_SIZE) as usize;
			while area[index] != 1 {
				index = rand::thread_rng().gen_range(0, AREA_SIZE * AREA_SIZE) as usize;
			}
			area[index] = 0;
		}

		if area[snake.position.y as usize * AREA_SIZE + snake.position.x as usize] > 1 {
			snake.is_alive = false;
		}

		for i in 0..area.len() {
			if (snake.position.y as usize * AREA_SIZE) + snake.position.x as usize == i {
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
	let mut snake = Snake::new("Joe".to_string());
	let mut area: Vec<u32> = vec![1; AREA_SIZE * AREA_SIZE];
	area[rand::thread_rng().gen_range(0, AREA_SIZE * AREA_SIZE) as usize] = 0;
	let mut buffer: Vec<u32> = vec![1; AREA_SIZE * AREA_SIZE];
	let mut window = Window::new(&"GL HF!".to_string(),
                            	AREA_SIZE,
                                AREA_SIZE,
                                WindowOptions {
                                    resize: false,
                                    scale: Scale::X32,
                                    ..WindowOptions::default()
                                })
	.expect("Unable to Open Window");
	let mut game_over = false;
	while window.is_open() && !window.is_key_down(minifb::Key::Escape){
		if snake.is_alive {
			for i in 0..area.len() {
				buffer[i] = match area[i] {
					0 => 0xFF00E436, // apple
					1 => 0xFFFFF1E8, // nothing
					_ => 0xFFFF004D, // snake "Joe" :^)
				};
			}
			Snake::logic(&mut snake, &mut area, &mut window);
			println!("x: {}, y: {}", &snake.position.x , &snake.position.y);
			std::thread::sleep(std::time::Duration::from_millis(100));
		}
		else if game_over == false{
			for i in 0..area.len() {
				buffer[i] = match area[i] {
						0 => 0xFF00E436, // apple
						1 => 0xFFFFF1E8, // nothing
						_ => 0xFF7E2553, // snake "Joe" :^)
				};
			}
			window.set_title(&"Game Over ;c".to_string());
			game_over = true;
		}
		window.update_with_buffer(&buffer).unwrap();
	}
}