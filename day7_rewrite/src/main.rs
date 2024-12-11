const INPUT: &str = include_str!("input.txt");

// -------------------------------------------------
//
// rewrite of day 7 to use less .clone()s to see how efficient it is (3-4 ms! not bad!)
// ( additional optimizations inspired by this solution on reddit:           )
// ( https://www.reddit.com/r/adventofcode/comments/1h8l3z5/comment/m0wq0ta/ )
//
// -------------------------------------------------


fn main() {
	use std::time::Instant;
    let now = Instant::now();

	/*
	let mut sum = 0;
	let mut sum2 = 0;
	
	for i in INPUT.lines() {
		let temp: Vec<&str> = i.split(": ").collect();
		// println!("{}", temp.get(0).unwrap().parse::<usize>().unwrap());
		let target: usize = temp.get(0).unwrap().parse::<usize>().unwrap();
		let nums: Vec<usize> = temp.get(1).unwrap().split(" ").map(|t| {
			t.parse::<usize>().unwrap()
		}).collect();
		let amt = nums.len();
		if can_equal(&nums, target, amt) {
			sum += target;
		}
		if can_equalb(&nums, target, amt) {
			sum2 += target;
		}
	}
	*/
	let sum = INPUT.lines().filter_map(|line| {
		let (target, nums) = line.split_once(": ")?;
		let target = target.parse::<usize>().unwrap();
		let nums = nums.split(" ").map(|t| {
			t.parse::<usize>().unwrap()
		}).collect();
		can_equal(&nums, target, nums.len()).then_some(target)
	}).sum::<usize>();
	
	let sum2 = INPUT.lines().filter_map(|line| {
		let (target, nums) = line.split_once(": ")?;
		let target = target.parse::<usize>().unwrap();
		let nums = nums.split(" ").map(|t| {
			t.parse::<usize>().unwrap()
		}).collect();
		can_equalb(&nums, target, nums.len()).then_some(target)
	}).sum::<usize>();
	
	// didnt actually speed it up like at all lol maybe its just my pc
	
	let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
	
    println!("Part A: {}", sum);
    println!("Part B: {}", sum2);
	
	
}

fn can_equal(nums: &Vec<usize>, res: usize, rem: usize) -> bool{
	if rem == 1 {
		return *nums.get(0).unwrap() == res
	} else {
		let last_num = *nums.get(rem-1).unwrap();
		return  (res % last_num == 0 && can_equalb(nums, res / last_num, rem - 1)) || 
			    (res > last_num && can_equalb(nums, res - last_num, rem - 1));
	}
}

fn can_equalb(nums: &Vec<usize>, res: usize, rem: usize) -> bool{
	if rem == 1 {
		return *nums.get(0).unwrap() == res
	} else {
		let last_num = *nums.get(rem-1).unwrap();
		let nh = get_next_hundred(last_num);
		return  (res % last_num == 0 && can_equalb(nums, res / last_num, rem - 1)) || 
			    (res > last_num && 
					(can_equalb(nums, res - last_num, rem - 1) || 
						((res - last_num) % nh == 0 && can_equalb(nums, (res - last_num) / nh, rem - 1))
					)
			    );
	}
}

fn get_next_hundred(num: usize) -> usize{
	let mut i: usize = 1;
	while i <= num {
		i = i * 10;
	}
	return i;
}