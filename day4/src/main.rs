const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut input_array: Vec<&str> = Vec::new();
	let mut cols = 0;
	
	let lines = INPUT.lines();
	for i in lines{
		let line: Vec<&str> = i.split("").collect();
		cols = line.len();
		for j in line{
			input_array.push(j);
		}
	}
	for i in 0..input_array.len(){
		print!("{}", input_array.get(i).unwrap());
		if i % cols == 0 {
			print!("\n");
		}
	}
	print!("\n");
	let mut words_found = 0;
	for i in 0..input_array.len(){
		let x = i % cols;
		let y = i / cols;
		
		let rows = input_array.len() / cols;
		
		if x >= 3 {
			if search_xmas(input_array.clone(), cols, x, y, -1, 0) {
				words_found+=1;
			}
			if y >= 3 {
				if search_xmas(input_array.clone(), cols, x, y, -1, -1) {
					words_found+=1;
				}
			}
			if y < rows - 3 {
				if search_xmas(input_array.clone(), cols, x, y, -1, 1) {
					words_found+=1;
				}
			}
		}
		if x < cols - 3 {
			if search_xmas(input_array.clone(), cols, x, y, 1, 0) {
				words_found+=1;
			}
			if y >= 3 {
				if search_xmas(input_array.clone(), cols, x, y, 1, -1) {
					words_found+=1;
				}
			}
			if y < rows - 3 {
				if search_xmas(input_array.clone(), cols, x, y, 1, 1) {
					words_found+=1;
				}
			}
		}
		if y >= 3 {
			if search_xmas(input_array.clone(), cols, x, y, 0, -1) {
				words_found+=1;
			}
		}
		if y < rows - 3 {
			if search_xmas(input_array.clone(), cols, x, y, 0, 1) {
				words_found+=1;
			}
		}
	}
	println!("Part A: {}", words_found);
	words_found = 0;
	for i in 0..input_array.len(){
		let x = i % cols;
		let y = i / cols;
		
		let rows = input_array.len() / cols;
		
		if x > 0 && x < cols - 1 && y > 0 && y < rows - 1 {
			if search_x_mas(input_array.clone(), cols, x, y) {
				words_found+=1;
			}
		}
	}
	println!("Part B: {}", words_found);
}

fn search_xmas(arr: Vec<&str>, cols: usize, x: usize, y: usize, dx: i32, dy: i32) -> bool{
	let l1 = arr.get(x + y * cols).unwrap();
	if *l1 != "X" {
		return false;
	}
	let l2 = arr.get(((x as i32 + dx) + (y as i32 + dy) * cols as i32) as usize).unwrap();
	if *l2 != "M" {
		return false;
	}
	let l3 = arr.get(((x as i32 + 2 * dx) + (y as i32 + 2 * dy) * cols as i32) as usize).unwrap();
	if *l3 != "A" {
		return false;
	}
	let l4 = arr.get(((x as i32 + 3 * dx) + (y as i32 + 3 * dy) * cols as i32) as usize).unwrap();
	if *l4 != "S" {
		return false;
	}
	return true;
} 

fn search_x_mas(arr: Vec<&str>, cols: usize, x: usize, y: usize) -> bool{
	let l1 = arr.get(x + y * cols).unwrap();
	if *l1 != "A" {
		return false;
	}
	let l2 = arr.get((x - 1) + (y + 1) * cols).unwrap();
	let l3 = arr.get((x + 1) + (y - 1) * cols).unwrap();
	if !(*l2 == "M" && *l3 == "S") && !(*l2 == "S" && *l3 == "M") {
		return false;
	}
	let l4 = arr.get((x + 1) + (y + 1) * cols).unwrap();
	let l5 = arr.get((x - 1) + (y - 1) * cols).unwrap();
	if !(*l4 == "M" && *l5 == "S") && !(*l4 == "S" && *l5 == "M") {
		return false;
	}
	return true;
}