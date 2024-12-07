const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut rules: Vec<(usize, usize)> = Vec::new();
	let mut mode = 0;
	let mut sum = 0;
	let mut sum2 = 0;
	for l in INPUT.lines(){
		if mode == 0 {
			if l.is_empty(){
				mode = 1;
			}else{
				let mut l_split = l.split("|");
				rules.push((l_split.next().unwrap().parse::<usize>().unwrap(), l_split.next().unwrap().parse::<usize>().unwrap()));
			}
		}else if mode == 1 {
			let mut pages: Vec<usize> = l.split(",").map(|p| {
				p.parse::<usize>().unwrap()
			}).collect();
			// println!("{:?}", pages);
			if is_valid_list(pages.clone(), rules.clone()) {
				sum += pages.get((pages.len()-1)/2).unwrap();
			}else{
				while !is_valid_list(pages.clone(), rules.clone()){
					let pgs = pages.clone();
					for i in 0..pgs.len()-1{
						let p1 = pgs.get(i).unwrap();
						for j in i+1..pgs.len(){
							let p2 = pgs.get(j).unwrap();
							for r in &rules{
								if r.0 == *p2 && r.1 == *p1 {
									pages.swap(i, j);
								}
							}
						}
					}
				}
				sum2 += pages.get((pages.len()-1)/2).unwrap();
			}
		}
	}
	println!("Part A: {}", sum);
	println!("Part B: {}", sum2);
}

fn is_valid_list(pgs: Vec<usize>, rls: Vec<(usize, usize)>) -> bool{
	for i in 0..pgs.len()-1{
		let p1 = pgs.get(i).unwrap();
		for j in i+1..pgs.len(){
			let p2 = pgs.get(j).unwrap();
			for r in &rls{
				if r.0 == *p2 && r.1 == *p1 {
					// println!("Rule broken: {} before {}", p1, p2);
					return false;
				}
			}
		}
	}
	return true;
}