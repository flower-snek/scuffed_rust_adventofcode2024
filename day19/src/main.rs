use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let testinput = include_str!("input_test.txt");
    println!("Part A TEST: {:?}", partA(testinput));
    println!("Part A REAL: {:?}", partA(input));
    println!("===============================");
    println!("Part B TEST: {:?}", partB(testinput));
    println!("Part B REAL: {:?}", partB(input));
}

fn partA(input: &str) -> u32 {
    let mut towels = vec![];
    let mut patterns = vec![];
    let mut lines = input.lines();
    towels = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    let _ = lines.next();
    for i in lines {
        patterns.push(i);
    }
    // now how do i do this recursively
    // edit: the answer is you dont.
    fn can_make(pattern: &str, towels: &Vec<&str>) -> bool {
        let mut stack = vec![pattern];
        let mut checked = vec![pattern];
        while stack.len() > 0 {
            let p = stack.pop().unwrap();
            for t in 0..towels.len() {
                let tt = *towels.get(t).unwrap();
                if p.starts_with(tt) {
                    let str = p.split_at(tt.len()).1;
                    if str == "" {
                        return true;
                    } else if !checked.contains(&str) {
                        stack.push(str);
                        checked.push(str);
                    }
                }
            }
        }
        false
    }
    let mut sum = 0;
    for p in 0..patterns.len() {
        // println!("{p}");
        if can_make(patterns[p], &towels) {
            sum += 1;
        }
    }
    sum
}

fn partB(input: &str) -> u64 {
    let mut towels = vec![];
    let mut patterns = vec![];
    let mut lines = input.lines();
    towels = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    lines.next();
    patterns = lines.collect();
    fn can_make(pattern: &str, towels: &Vec<&str>) -> u64 {
        let plen: usize = pattern.len();
        let mut count = vec![0; plen+1];
        count[0] = 1;
        for i in 0..plen {
            let p = pattern.split_at(i).1;
            let pc = count[i];
            for t in 0..towels.len() {
                let tt = *towels.get(t).unwrap();
                if p.starts_with(tt) {
                    count[i + tt.len()] += pc;
                }
            }
        }
        count[plen]
    }
    patterns.iter().map(|&p| can_make(p, &towels)).sum::<u64>()
}