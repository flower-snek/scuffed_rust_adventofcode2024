use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
	let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
	let mut y = 0;
	let mut x = 0;
	for i in INPUT.lines() {
		x = 0;
		for j in i.chars() {
			if j != '.' {
				let freq_vec = frequencies.entry(j).or_insert(Vec::new());
				freq_vec.push((x, y));
			}
			x+=1;
		}
		y+=1;
	}
	let (w, h) = (x, y);
	let mut map: HashMap<(usize, usize), usize> = HashMap::new();
	let mut mapb: HashMap<(usize, usize), usize> = HashMap::new();
	for val in frequencies.values() {
		// println!("{val:?}");
		for i in 0..val.len()-1 {
			for j in i+1..val.len() {
				let p1 = val[i];
				let p2 = val[j];
				
				// part A
				
				if p1.0 * 2 >= p2.0 && p1.1 * 2 >= p2.1 {
					let anti1 = (p1.0 * 2 - p2.0, p1.1 * 2 - p2.1);
					if anti1.0 < w && anti1.1 < h {
						let me1 = map.entry(anti1).or_insert(0);
						*me1 += 1;
					}
				}
				if p2.0 * 2 >= p1.0 && p2.1 * 2 >= p1.1 {
					let anti2 = (p2.0 * 2 - p1.0, p2.1 * 2 - p1.1);
					if anti2.0 < w && anti2.1 < h {
						let me2 = map.entry(anti2).or_insert(0);
						*me2 += 1;
					}
				}
				
				// part B
				
				// augh im using unsigned ints
				// time to convert everything to signed ints
				// (or at least just the dx/dy)
				let (dx, dy) = ((p1.0 as i32) - (p2.0 as i32), (p1.1 as i32) - (p2.1 as i32));
				let mut temp_pos = (p1.0 as i32, p1.1 as i32);
				while temp_pos.0 >= 0 && temp_pos.1 >= 0 && temp_pos.0 < w as i32 && temp_pos.1 < h as i32 {
					let me1 = mapb.entry((temp_pos.0 as usize, temp_pos.1 as usize)).or_insert(0);
					*me1 += 1;
					temp_pos = (temp_pos.0 + dx, temp_pos.1 + dy);
				}
				temp_pos = (p2.0 as i32, p2.1 as i32);
				while temp_pos.0 >= 0 && temp_pos.1 >= 0 && temp_pos.0 < w as i32 && temp_pos.1 < h as i32 {
					let me1 = mapb.entry((temp_pos.0 as usize, temp_pos.1 as usize)).or_insert(0);
					*me1 += 1;
					temp_pos = (temp_pos.0 - dx, temp_pos.1 - dy);
				}
			}
		}
	}
	// println!("{}, {}", w, h);
	/*
	for val in mapb.keys(){
		println!("{val:?}");
	}*/
	println!("Part A: {}", map.values().collect::<Vec<_>>().len());
	println!("Part B: {}", mapb.values().collect::<Vec<_>>().len());
    // println!("Hello, world!");
}
