use regex::Regex;
const INPUT: &str = include_str!("input.txt");

fn main() {
    let re = Regex::new(r"mul\((?<a>[0-9]*),(?<b>[0-9]*)\)").unwrap();
	let sum: Vec<usize> = re.captures_iter(INPUT).map(|caps| {
		let a = caps.name("a").unwrap().as_str().parse::<usize>().unwrap();
		let b = caps.name("b").unwrap().as_str().parse::<usize>().unwrap();
		a * b
	}).collect();
	println!("Part A: {}", sum.clone().into_iter().sum::<usize>());
	
	let do_regex = Regex::new(r"do\(\)").unwrap();
	let dont_regex = Regex::new(r"don\'t\(\)").unwrap();
	
	let sum_loc: Vec<_> = re.captures_iter(INPUT).map(|caps| {
		caps.get(0).unwrap().start()
	}).collect();
	let do_loc: Vec<_> = do_regex.captures_iter(INPUT).map(|caps| {
		caps.get(0).unwrap().start()
	}).collect();
	let dont_loc: Vec<_> = dont_regex.captures_iter(INPUT).map(|caps| {
		caps.get(0).unwrap().start()
	}).collect();
	
	let mut active = true;
	let mut sum2 = 0;
	for i in 0..INPUT.len(){
		for a in 0..do_loc.len(){
			if do_loc.get(a).unwrap() == &i {
				active = true
			}
		}
		for a in 0..dont_loc.len(){
			if dont_loc.get(a).unwrap() == &i {
				active = false
			}
		}
		if active {
			for a in 0..sum_loc.len(){
				if sum_loc.get(a).unwrap() == &i {
					sum2 += sum.get(a).unwrap();
				}
			}
		}
	}
	
	// let test = do_res.next();
	println!("Part B: {}", sum2);
}
