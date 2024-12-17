

fn main() {
    let input = include_str!("input.txt");
    let testinput = include_str!("input_test.txt");
    println!("Part A TEST: {:?}", partA(testinput));
    println!("Part A REAL: {:?}", partA(input));
    println!("===============================");
    // println!("Part B TEST: {}", partB(testinput));
    println!("Part B REAL: {}", partB(input));
}

fn partA(input: &str) -> Vec<u64> {

    let mut registers = (0,0,0);
    let mut outputs: Vec<u64> = vec![];
    let mut ilines = input.lines();
    let rega_str = ilines.next().unwrap();
    let regb_str = ilines.next().unwrap();
    let regc_str = ilines.next().unwrap();
    let _ = ilines.next().unwrap();
    let program_str = ilines.next().unwrap();

    registers.0 = rega_str[12..].parse::<u64>().unwrap();
    registers.1 = regb_str[12..].parse::<u64>().unwrap();
    registers.2 = regc_str[12..].parse::<u64>().unwrap();
    // println!("{registers:?}");
    let program_vals: Vec<u64> = program_str[9..].split(',').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    fn combo_od(n: u64, reg: (u64, u64, u64)) -> u64 {
        if n <= 3 {
            n
        }else if n == 4 {
            reg.0
        }else if n == 5 {
            reg.1
        }else {
            reg.2
        }
    }
    let mut ptr: usize = 0;
    while ptr < program_vals.len() {
        let op = program_vals[ptr];
        let od = program_vals[ptr+1];
        let mut inc = true;
        // println!("{:?}, {op}, {od}", registers);
        match op {
            0 => {
                // adv
                registers.0 = registers.0 / (2_u64.pow(combo_od(od, registers) as u32));
            },
            1 => {
                // bxl
                registers.1 = registers.1 ^ od;
            },
            2 => {
                // bst
                registers.1 = combo_od(od, registers) % 8;
            },
            3 => {
                // jnz
                if registers.0 != 0 {
                    ptr = od as usize;
                    inc = false;
                }
            },
            4 => {
                // bxc
                registers.1 = registers.1 ^ registers.2;
            },
            5 => {
                // out
                outputs.push(combo_od(od, registers) % 8);
            },
            6 => {
                // bdv
                registers.1 = registers.0 / (2_u64.pow(combo_od(od, registers) as u32));
            },
            7 => {
                // cdv
                registers.2 = registers.0 / (2_u64.pow(combo_od(od, registers) as u32));
            },
            _ => panic!("something's wrong"),
        }
        if inc {
            ptr += 2;
        }
    }


    outputs
}

fn partB(input: &str) -> u64 {

    let mut registers = (0,0,0);
    let mut outputs: Vec<u64> = vec![];
    let mut ilines = input.lines();
    let rega_str = ilines.next().unwrap();
    let regb_str = ilines.next().unwrap();
    let regc_str = ilines.next().unwrap();
    let _ = ilines.next().unwrap();
    let program_str = ilines.next().unwrap();

    registers.0 = rega_str[12..].parse::<u64>().unwrap();
    registers.1 = regb_str[12..].parse::<u64>().unwrap();
    registers.2 = regc_str[12..].parse::<u64>().unwrap();
    // println!("{registers:?}");
    let program_vals: Vec<u64> = program_str[9..].split(',').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    fn combo_od(n: u64, reg: (u64, u64, u64)) -> u64 {
        if n <= 3 {
            n
        }else if n == 4 {
            reg.0
        }else if n == 5 {
            reg.1
        }else {
            reg.2
        }
    }
    let mut ptr: i32 = program_vals.len() as i32 - 2;
    let mut cur_output_goal: i32 = program_vals.len() as i32 - 1;
    let mut jump_from = 0;
    let mut jump_to = 0;
    registers.0 = 0;
    while cur_output_goal >= 0 || ptr >= 0 {
        let op = program_vals[ptr as usize];
        let od = program_vals[ptr as usize +1];
        // println!("{:?}, {op}, {od}, {cur_output_goal}", registers);
        match op {
            0 => {
                // adv
                // reverse: multiply to get min
                registers.0 = registers.0 * (2_u64.pow(combo_od(od, registers) as u32));
            },
            1 => {
                // bxl
                // reverse: still xor
                registers.1 = registers.1 ^ od;
            },
            2 => {
                // bst
                // reverse: update register that its being read from
                // registers.1 = combo_od(od, registers) % 8;
                if od == 4 {
                    registers.0 += registers.1 - (registers.0 % 8)
                }
                if od == 5 {
                    // uh this is wrong
                    registers.1 += registers.1 - (registers.1 % 8)
                }
                if od == 6 {
                    registers.2 += registers.1 - (registers.2 % 8)
                }
            },
            3 => {
                // jnz
                // reverse: need to set a jump_from and jump_to point
                jump_from = od as i32;
                jump_to = ptr;
            },
            4 => {
                // bxc
                // reverse: still xor
                registers.1 = registers.1 ^ registers.2;

            },
            5 => {
                // out
                // reverse: set register based on expected output
                // outputs.push(combo_od(od, registers) % 8);
                let o = program_vals[cur_output_goal as usize];
                if od == 4 {
                    registers.0 += o;
                }
                if od == 5 {
                    registers.1 += o;
                }
                if od == 6 {
                    registers.2 += o;
                }
                cur_output_goal -= 1;
            },
            6 => {
                // bdv
                // reverse: like adv
                // registers.1 = registers.0 * (2_u64.pow(combo_od(od, registers) as u32));
                // actually when its not adv its more like a limit on A.
                registers.0 = registers.1 * (2_u64.pow(combo_od(od, registers) as u32));
            },
            7 => {
                // cdv
                // C = A / 2^whatever
                // thus given C, A = C * 2^whatever
                registers.0 = registers.2 * (2_u64.pow(combo_od(od, registers) as u32));
            },
            _ => panic!("something's wrong"),
        }
        if ptr == jump_from && cur_output_goal >= 0 {
            ptr = jump_to;
            /*
            if registers.0 == 0 {
                registers.0 += 8;
            }*/
        }else{
            ptr -= 2;
        }
    }
    // jank brute force solution
    let mut cur_A:u64 = 0;
    for cog in (0..program_vals.len()).rev() {
        cur_A = cur_A * 8;
        // find an A for which the weird equation is true
        let sol = program_vals[cog];
        // println!("{sol}");
        let mut new = cur_A;
        for na in (cur_A..(cur_A + 8)).rev() {
            let b = na % 8;
            let c = na / 2_u64.pow((b ^ 3) as u32);
            if ((((b) ^ 3) ^ 4) ^ c) % 8 == sol {
                new = na;
            }
        }
        cur_A = new;
        // let b = cur_A % 8;
        // let c = cur_A / 2_u64.pow(b as u32);
        // println!("{}, {}, {}, {}", b, (b) ^ 3, ((b) ^ 3) ^ 4, ((b ^ 3) ^ 4) ^ c);
        // println!("{cur_A}");
    }

    cur_A
}

//Program: 2,4,1,3,7,5,0,3,1,4,4,7,5,5,3,0
// B = A % 8
// B = B ^ 3
// C = A / 2^B -- but if C = 0 after this step then it doesnt really work so maybe i should assume some C?
// A = A / 8
// B = B ^ 4
// B = B ^ C
// OUT B
// JNZ 0
// so (A % 8) ^ (3 ^ 4 ^ A/2^(A%8)) == OUT