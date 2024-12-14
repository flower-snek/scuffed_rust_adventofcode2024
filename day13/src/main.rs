fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut machines: Vec<Machine> = vec![];
    let mut machines2: Vec<Machine> = vec![];
    for i in 0..input.len() {
        if i % 4 == 0 {
            let ba_str:&str = &(input.get(i).unwrap())[12..];
            let ba_vals = ba_str.split_once(", Y+").unwrap();
            let (ax, ay) = (ba_vals.0.parse::<i64>().unwrap(), ba_vals.1.parse::<i64>().unwrap());

            let bb_str:&str = &(input.get(i+1).unwrap())[12..];
            let bb_vals = bb_str.split_once(", Y+").unwrap();
            let (bx, by) = (bb_vals.0.parse::<i64>().unwrap(), bb_vals.1.parse::<i64>().unwrap());

            let bp_str:&str = &(input.get(i+2).unwrap())[9..];
            let bp_vals = bp_str.split_once(", Y=").unwrap();
            let (px, py) = (bp_vals.0.parse::<i64>().unwrap(), bp_vals.1.parse::<i64>().unwrap());

            let m = Machine {
                AX: ax,
                AY: ay,
                BX: bx,
                BY: by,
                PX: px,
                PY: py
            };
            machines.push(m);
            let m2 = Machine {
                AX: ax,
                AY: ay,
                BX: bx,
                BY: by,
                PX: px + 10000000000000_i64,
                PY: py + 10000000000000_i64,
            };
            machines2.push(m2);
        }
    }
    let mut t = 0;
    for m in machines {
        let lower = m.AX * m.BY - m.AY * m.BX;
        let upper = m.PX * m.BY - m.PY * m.BX;
        let (ub, lb) = (m.PX - (upper/lower) * m.AX, m.BX);
        if upper % lower == 0 && ub % lb == 0 {
            // println!("{}, {}", upper/lower, ub/lb);
            t += upper/lower*3 + ub/lb;
        }

        if lower == 0 {
            // println!("lower == 0");
        }else {
            // println!("{}, {}", upper / lower, upper % lower);
        }
    }
    println!("Part A: {t}");
    let mut t = 0;
    for m in machines2 {
        let lower = m.AX * m.BY - m.AY * m.BX;
        let upper = m.PX * m.BY - m.PY * m.BX;
        let (ub, lb) = (m.PX - (upper/lower) * m.AX, m.BX);
        if upper % lower == 0 && ub % lb == 0 {
            // println!("{}, {}", upper/lower, ub/lb);
            t += upper/lower*3 + ub/lb;
        }

        if lower == 0 {
            // println!("lower == 0");
        }else {
            // println!("{}, {}", upper / lower, upper % lower);
        }
    }
    println!("Part B: {t}");
}


struct Machine {
    AX: i64,
    AY: i64,
    BX: i64,
    BY: i64,
    PX: i64,
    PY: i64,
}