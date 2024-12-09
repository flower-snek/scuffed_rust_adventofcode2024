const INPUT: &str = include_str!("input.txt");

fn main() {
	let mut filesystem: Vec<i64> = Vec::new();
	let mut pos: usize = 0;
	for i in INPUT.lines().next().unwrap().chars() {
		// println!("{}", i);
		let amt = i.to_digit(10).unwrap();
		for _ in 0..amt {
			if pos % 2 == 0 {
				filesystem.push((pos/2) as i64);
			}else {
				filesystem.push(-1);
			}
		}
		pos+=1;
	}
	pos = 0;
	let mut end_pos: usize = filesystem.len() - 1;
	let mut sum: i64 = 0;
	while pos <= end_pos {
		if *filesystem.get(pos).unwrap() == -1 {
			while end_pos >= pos && *filesystem.get(end_pos).unwrap() == -1 {
				end_pos -= 1;
			}
			if *filesystem.get(end_pos).unwrap() > -1 {
				sum += *filesystem.get(end_pos).unwrap() * pos as i64;
				// println!("{} @ pos {}", *filesystem.get(end_pos).unwrap(), pos);
				end_pos -= 1;
			}
		}else {
			
			sum += *filesystem.get(pos).unwrap() * pos as i64;
			// println!("{} @ pos {}", *filesystem.get(pos).unwrap(), pos);
		}
		pos += 1;
	}
    println!("Part A: {}", sum);
	
	// block representation
	let mut filesystem2: Vec<(i64, i64, i64)> = Vec::new(); // id, position, size
	pos = 0;
	end_pos = 0; // just reusing variables bc lazy lol (this is block position in the fs)
	for i in INPUT.lines().next().unwrap().chars() {
		// println!("{}", i);
		let amt = i.to_digit(10).unwrap() as usize;
		if amt > 0 {
			if pos % 2 == 0 {
				filesystem2.push(((pos/2) as i64, end_pos as i64, amt as i64));
			}else {
				filesystem2.push((-1, end_pos as i64, amt as i64));
			}
		}
		pos+=1;
		end_pos += amt;
	}
	
	end_pos = filesystem2.len(); // now end_pos is the current file id (going backwards)
	let mut sumb = 0;
	while end_pos > 0 {
		end_pos -= 1;
		let mut end_block = filesystem2[end_pos];
		if end_block.0 != -1 {
			pos = 0;
			let mut found_block = false;
			while !found_block && pos < end_pos {
				let mut pos_block = filesystem2[pos];
				if pos_block.0 == -1 && pos_block.2 >= end_block.2 {
					// it fits!
					end_block.1 = pos_block.1;
					pos_block.1 = pos_block.1 + end_block.2;
					pos_block.2 -= end_block.2;
					found_block = true;
					filesystem2[pos] = pos_block;
					filesystem2[end_pos] = end_block;
					// println!("file id {} moved to {}", end_block.0, end_block.1);
					// println!("empty block at {} updated to pos {} size {}", end_block.1, pos_block.1, pos_block.2);
				}
				pos += 1
			}
			// whatever happened there, we now add end_block since nothing will change its place
			for i in end_block.1..(end_block.1+end_block.2) {
				sumb += end_block.0 * i;
			}
			// println!("{:?}", filesystem2);
		}
	}
	// now sum
	println!("Part B: {}", sumb);
}
