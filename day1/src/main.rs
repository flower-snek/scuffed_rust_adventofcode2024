
const INPUT: &str = include_str!("input.txt");

fn main() {
	let lines = INPUT.lines();
	let mut first_nums: Vec<i32> = Vec::new();
	let mut second_nums: Vec<i32> = Vec::new();
	for l in lines {
		let mut sp = l.split("   ");
		first_nums.push(sp.next().unwrap().parse::<i32>().unwrap());
		second_nums.push(sp.next().unwrap().parse::<i32>().unwrap());
	}
	first_nums.sort();
	second_nums.sort();
	let mut sum = 0;
	for i in 0..first_nums.len(){
		sum = sum + (first_nums.get(i).expect("") - second_nums.get(i).expect("")).abs();
	}
    println!("Part A: {sum}");
	
	let mut sim = 0;
	for i in 0..first_nums.len(){
		let n1 = first_nums.get(i).expect("");
		for j in 0..second_nums.len(){
			let n2 = second_nums.get(j).expect("");
			if n1 == n2 {
				sim += n1;
			}
		}
	}
	println!("Part B: {sim}");
}
