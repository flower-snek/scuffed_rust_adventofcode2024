use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let testinput = include_str!("input_test.txt");
    println!("Part A TEST: {:?}", part_a(testinput));
    println!("Part A REAL: {:?}", part_a(input));
    println!("===============================");
    println!("Part B TEST: {:?}", part_b(testinput));
    println!("Part B REAL: {:?}", part_b(input));
}

fn part_a(input: &str) -> u64 {
    // let secret_start: Vec<_> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    fn next_secret(n: u64) -> u64 {
        let s1 = (n ^ (n * 64)) % 16777216;
        let s2 = (s1 ^ (s1 / 32)) % 16777216;
        (s2 ^ (s2 * 2048)) % 16777216
    }
    input.lines().map(|s| {
        let mut n = s.parse::<u64>().unwrap();
        for _ in 0..2000 {
            n = next_secret(n);
        }
        n
    }).sum::<u64>()
}

fn part_b(input: &str) -> u64 {
    fn next_secret(n: u64) -> u64 {
        let s1 = (n ^ (n * 64)) % 16777216;
        let s2 = (s1 ^ (s1 / 32)) % 16777216;
        (s2 ^ (s2 * 2048)) % 16777216
    }
    let mut price_differences: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();
    for s in input.lines() {
        let mut n = s.parse::<u64>().unwrap();
        let mut local_pd_hash: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();
        let mut prev_one = n % 10;
        let mut prev_deltas = (0, 0, 0, 0);
        for i in 0..2000 {
            n = next_secret(n);
            prev_deltas = (prev_deltas.1, prev_deltas.2, prev_deltas.3, (n % 10) as i32 - prev_one as i32);
            prev_one = n % 10;
            if i >= 3 {
                let e = local_pd_hash.entry(prev_deltas).or_insert(prev_one);
            }
        }
        for (diffs, first) in &local_pd_hash {
            let e = price_differences.entry(*diffs).or_insert(0);
            *e += *first;
        }
    }
    let mut max = 0;
    for (diffs, count) in &price_differences {
        if max < *count {
            max = *count;
        }
    }
    max
}