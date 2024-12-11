const INPUT: &str = include_str!("input.txt");

// afterthought: there was probably a cleaner way to do this lol

fn main() {
	let mut map: Vec<u32> = Vec::new();
	let mut width = 0;
	for l in INPUT.lines(){
		for c in l.chars(){
			if c == '.' {
				map.push(99);
			}else {
				map.push(c.to_digit(10).unwrap());
			}
		}
		width = l.len() as usize;
	}
	// assuming square
	
	let mut sum = 0;
	let mut sumb = 0;
	for i in 0..map.len() {
		/*
		print!("{}", map.get(i).unwrap());
		if i % width == width-1 {
			print!("\n");
		}
		*/
		if *map.get(i).unwrap() == 0 {
			sum += find_score(&map, &width, i % width, i / width);
			sumb += find_scoreb(&map, &width, i % width, i / width);
		}
	}
	println!("Part A: {}", sum);
	println!("Part B: {}", sumb);
}

fn find_score(map: &Vec<u32>, width: &usize, x: usize, y: usize) -> usize {
	let mut positions: Vec<(usize, usize)> = vec![(x, y)];
	let mut height = 0;
	while height < 9 {
		let mut new_positions: Vec<(usize, usize)> = vec![];
		for &(_x, _y) in &positions {
			if _x > 0 {
				if map[(_x - 1) + _y * *width] == height + 1 {
					if !new_positions.contains(&(_x - 1, _y)) {
						new_positions.push((_x - 1, _y));
					}
				}
			}
			if _x < *width - 1 {
				if map[(_x + 1) + _y * *width] == height + 1 {
					if !new_positions.contains(&(_x + 1, _y)) {
						new_positions.push((_x + 1, _y));
					}
				}
			}
			if _y > 0 {
				if map[_x + (_y - 1) * *width] == height + 1 {
					if !new_positions.contains(&(_x, _y - 1)) {
						new_positions.push((_x, _y - 1));
					}
				}
			}
			if _y < *width - 1 {
				if map[_x + (_y + 1) * *width] == height + 1 {
					if !new_positions.contains(&(_x, _y + 1)) {
						new_positions.push((_x, _y + 1));
					}
				}
			}
		}
		
		positions = new_positions;
		height+=1;
	}
	return positions.len();
}

fn find_scoreb(map: &Vec<u32>, width: &usize, x: usize, y: usize) -> usize {
	let mut positions: Vec<(usize, usize)> = vec![(x, y)];
	let mut height = 0;
	while height < 9 {
		let mut new_positions: Vec<(usize, usize)> = vec![];
		for &(_x, _y) in &positions {
			if _x > 0 {
				if map[(_x - 1) + _y * *width] == height + 1 {
					new_positions.push((_x - 1, _y));
				}
			}
			if _x < *width - 1 {
				if map[(_x + 1) + _y * *width] == height + 1 {
					new_positions.push((_x + 1, _y));
				}
			}
			if _y > 0 {
				if map[_x + (_y - 1) * *width] == height + 1 {
					new_positions.push((_x, _y - 1));
				}
			}
			if _y < *width - 1 {
				if map[_x + (_y + 1) * *width] == height + 1 {
					new_positions.push((_x, _y + 1));
				}
			}
		}
		
		positions = new_positions;
		height+=1;
	}
	return positions.len();
}