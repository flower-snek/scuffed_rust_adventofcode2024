const INPUT: &str = include_str!("input.txt");
fn main() {
	let mut walls: Vec<(usize, usize)> = Vec::new();
	let mut guard_start: (usize, usize) = (0, 0);
	let mut dir = 0;
	//    0
	//   3 1
	//    2
	let mut y = 1;
	let height = INPUT.lines().count();
	let mut width = 0;
	for i in INPUT.lines(){
		if y == 1 {
			width = i.len();
		}
		let mut x = 0;
		for j in i.split(""){
			if j == "#" {
				walls.push((x, y));
			}
			if j == "^" {
				guard_start = (x, y);
			}
			x+=1;
		}
		y+=1;
	}
	//println!("{:?}", walls);
	//println!("{:?}", guard);
	//println!("{} x {}", width, height);
	let mut guard = guard_start.clone();
	let mut map: Vec<bool> = Vec::new();
	let mut mapdir: Vec<usize> = Vec::new();
	for _ in 0..width*height{
		map.push(false);
		mapdir.push(9);
	}
	let mut sum2 = 0;
	while guard.0 <= width && guard.0 > 0 && guard.1 <= height && guard.1 > 0 {
		let mut next_pos: (usize, usize) = (guard.0, guard.1);
		if dir == 0 {
			next_pos.1 -= 1;
		}
		if dir == 1 {
			next_pos.0 += 1;
		}
		if dir == 2 {
			next_pos.1 += 1;
		}
		if dir == 3 {
			next_pos.0 -= 1;
		}
		let mut crash = false;
		for i in &walls{
			if next_pos.0 == i.0 && next_pos.1 == i.1 {
				crash = true;
			}
		}
		if crash {
			dir = (dir + 1) % 4
		}else {
			if mapdir[(guard.0 - 1) + (guard.1 - 1) * width] == (dir + 1)%4 {
				println!("{}, {} loop block", next_pos.0, next_pos.1);
				sum2 += 1;
			}
			map[(guard.0 - 1) + (guard.1 - 1) * width] = true;
			mapdir[(guard.0 - 1) + (guard.1 - 1) * width] = dir;
			
			// make it so that all before and after also have the dir changed if it hasnt already been done
			// x axis
			let mut x_walls = (1, width+1);
			let y_walls = (1, height+1);
			for i in &walls{
				if i.0 == guard.0 {
					if i.1 < guard.1 {
						y_walls.0 == i.1;
					}else if y_walls.1 == height {
						y_walls.1 == i.1;
					}
				}
				if i.1 == guard.1 {
					if i.0 < guard.0 {
						x_walls.0 = i.0;
					}else if x_walls.1 == width {
						x_walls.1 = i.0;
					}
				}
			}
			if dir == 1 || dir == 3 {
				for x2 in x_walls.0..x_walls.1 {
					if mapdir[(x2 - 1) + (guard.1 - 1) * width] == 9 {
						mapdir[(x2 - 1) + (guard.1 - 1) * width] = dir;
					}
				}
			}else {
				for y2 in y_walls.0..y_walls.1 {
					if mapdir[(guard.0 - 1) + (y2 - 1) * width] == 9 {
						mapdir[(guard.0 - 1) + (y2 - 1) * width] = dir;
					}
				}
			}
			
			guard.0 = next_pos.0;
			guard.1 = next_pos.1;
		}
		// println!("{}, {}", guard.0, guard.1);
	}
	/*
	let mut x = 0;
	for i in mapdir {
		print!("{}", i);
		x+=1;
		if x % width == 0 {
			print!("\n");
		}
	}
	*/
	let mut sum = 0;
	for i in &map{
		if *i {
			sum+=1;
		}
	}
	println!("Part A: {}", sum);
	
	// ugh just brute force it
	sum2 = 0;
	let mut m = 0;
	for i in &map {
		if *i {
			let x = (m % width) + 1;
			let y = (m / width) + 1;
			// println!("{}, {}", x, y);
			let mut looping = false;
			guard = guard_start.clone();
			mapdir = Vec::new();
			dir = 0;
			for _ in 0..width*height{
				mapdir.push(9);
			}
			while guard.0 <= width && guard.0 > 0 && guard.1 <= height && guard.1 > 0 && !looping {
				let mut next_pos: (usize, usize) = (guard.0, guard.1);
				if dir == 0 {
					next_pos.1 -= 1;
				}
				if dir == 1 {
					next_pos.0 += 1;
				}
				if dir == 2 {
					next_pos.1 += 1;
				}
				if dir == 3 {
					next_pos.0 -= 1;
				}
				let mut crash = false;
				for i in &walls{
					if next_pos.0 == i.0 && next_pos.1 == i.1 {
						crash = true;
					}
				}
				if next_pos.0 == x && next_pos.1 == y {
					crash = true;
				}
				if crash {
					dir = (dir + 1) % 4
				}else {
					if mapdir[(guard.0 - 1) + (guard.1 - 1) * width] == dir {
						looping = true;
						sum2 += 1;
						println!("{}, {}", x, y);
					}
					mapdir[(guard.0 - 1) + (guard.1 - 1) * width] = dir;

					guard.0 = next_pos.0;
					guard.1 = next_pos.1;
				}
				// println!("{}, {}", guard.0, guard.1);
			}
		}
		m+=1;
	}
	
	println!("Part B: {}", sum2);
}