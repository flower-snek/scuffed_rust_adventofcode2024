use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

// day 1 using RustRover ide instead of notepad++

fn main() {
    // part A
    let mut stones = vec![];
    for i in INPUT.split(" ") {
        stones.push(i.parse::<u64>().unwrap())
    }
    for _ in 0..25 {
        // println!("{stones:?}");
        let mut new_stones = vec![];
        for i in stones {
            if i == 0 {
                new_stones.push(1);
            }else{
                let d = digits(i);
                // println!("{i}, {d}");
                if d % 2 == 0 {
                    let a = i % 10u64.pow((d/2).try_into().unwrap());
                    let b = i / 10u64.pow((d/2).try_into().unwrap());
                    // println!("{a}, {b}, {i}");
                    new_stones.push(a);
                    new_stones.push(b);
                }else{
                    new_stones.push(i * 2024);
                }
            }
        }
        stones = new_stones;
        // println!("{stones:?}");
    }
    println!("Part A: {}", stones.len());
    // println!("Part A: {:?}", stones);

    // part B
    let mut hstones: HashMap<u64, u64> = HashMap::new();
    for i in INPUT.split(" ") {
        let j = i.parse::<u64>().unwrap();
        let hash = hstones.entry(j).or_insert(0);
        *hash += 1;
    }
    for _ in 0..75 {
        // println!("{stones:?}");
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (v, n) in hstones.iter() {
            if *v == 0 {
                let hash = new_stones.entry(1).or_insert(0);
                *hash += n;
            }else{
                let d = digits(*v);
                // println!("{i}, {d}");
                if d % 2 == 0 {
                    let a = v % 10u64.pow((d/2).try_into().unwrap());
                    let b = v / 10u64.pow((d/2).try_into().unwrap());
                    // println!("{a}, {b}, {i}");
                    let hash = new_stones.entry(a).or_insert(0);
                    *hash += n;
                    let hash = new_stones.entry(b).or_insert(0);
                    *hash += n;
                }else{
                    let hash = new_stones.entry(v * 2024).or_insert(0);
                    *hash += n;
                }
            }
        }
        hstones = new_stones;
        // println!("{stones:?}");
    }
    println!("Part B: {}", hstones.values().sum::<u64>())
}

fn digits(n: u64) -> u64 {
    let mut i:u64 = 0;
    while 10u64.pow(i.try_into().unwrap()) - 1 < n {
        i+=1;
    }
    i
}