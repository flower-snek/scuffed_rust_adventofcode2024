const INPUT: &str = include_str!("input.txt");

fn main() {
	use std::time::Instant;
    let now = Instant::now();

	let mut sum = 0;
	let mut sum2 = 0;
	for i in INPUT.lines() {
		let temp: Vec<&str> = i.split(": ").collect();
		// println!("{}", temp.get(0).unwrap().parse::<usize>().unwrap());
		let target: usize = temp.get(0).unwrap().parse::<usize>().unwrap();
		let nums: Vec<usize> = temp.get(1).unwrap().split(" ").map(|t| {
			t.parse::<usize>().unwrap()
		}).collect();
		if can_equal(nums, target) {
			sum += target;
		}
		if can_equalb(nums, target) {
			sum2 += target;
		}
	}
    println!("Part A: {}", sum);
    println!("Part B: {}", sum2);
	
	let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn can_equal(nums: Vec<usize>, res: usize) -> bool{
	let mut nums_t = nums.clone();
	if nums.len() == 1 {
		return *nums.get(0).unwrap() == res
	} else {
		let last_num = nums_t.pop().unwrap();
		let mut result = false;
		if res % last_num == 0 {
			result = result || can_equal(nums_t.clone(), res / last_num);
		}
		if res > last_num {
			result = result || can_equal(nums_t.clone(), res - last_num);
		}
		return result;
	}
}

fn can_equalb(nums: Vec<usize>, res: usize) -> bool{
	let mut nums_t = nums.clone();
	if nums.len() == 1 {
		return *nums.get(0).unwrap() == res
	} else {
		let last_num = nums_t.pop().unwrap();
		let mut result = false;
		if res % last_num == 0 {
			result = result || can_equalb(nums_t.clone(), res / last_num);
		}
		if res > last_num {
			result = result || can_equalb(nums_t.clone(), res - last_num);
			if (res - last_num) % get_next_hundred(last_num) == 0 {
				// println!("{}, {}, {}", res, last_num, (res - last_num) / get_next_hundred(last_num));
				result = result || can_equalb(nums_t.clone(), (res - last_num) / get_next_hundred(last_num));
			}
		}
		return result;
	}
}

fn get_next_hundred(num: usize) -> usize{
	let mut i: usize = 1;
	while i <= num {
		i = i * 10;
	}
	return i;
}