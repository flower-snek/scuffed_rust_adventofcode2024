


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
    let mut map = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut size = (0, 0);
    let mut y = 0;
    for i in input.lines() {
        let mut x = 0;
        for j in i.chars() {
            if j == 'S' {
                start = (x, y);
            }
            if j == 'E' {
                end = (x, y);
            }
            if j == '#' {
                map.push(-1);
            } else {
                map.push(0);
            }
            x+=1;
        }
        size = (x, 0);
        y+=1;
    }
    size = (size.0, y);

    // dfs to fill out map
    let mut stack = vec![start.clone()];
    while stack.len() > 0 {
        // check adjacent
        let s = stack.pop().unwrap();
        let v = s.0 + s.1 * size.0;
        let new = map[v] + 1;
        if s.0 > 0 {
            if map[v-1] == 0 && (s.0 - 1 != start.0 || s.1 != start.1) {
                stack.push((s.0 - 1, s.1));
                map[v-1] = new;
            }
        }
        if s.0 < size.0 - 1 {
            if map[v+1] == 0 && (s.0 + 1 != start.0 || s.1 != start.1) {
                stack.push((s.0 + 1, s.1));
                map[v+1] = new;
            }
        }
        if s.1 > 0 {
            if map[v-size.0] == 0 && (s.0 != start.0 || s.1 -1  != start.1) {
                stack.push((s.0, s.1 - 1));
                map[v-size.0] = new;
            }
        }
        if s.1 < size.1 - 1 {
            if map[v+size.0] == 0 && (s.0 != start.0 || s.1 + 1 != start.1) {
                stack.push((s.0, s.1 + 1));
                map[v+size.0] = new;
            }
        }
    }
    // check val of all spaces 2 away from a map position
    let mut delta_size = vec![];
    for dx in -2..=2i32 {
        for dy in -2..=2i32 {
            if dx.abs() + dy.abs() == 2 {
                delta_size.push((dx, dy));
            }
        }
    }
    let mut sum = 0;
    let min_skip = 100;
    for i in 0..map.len() {
        if map[i] >= 0 {
            let pos = ((i % size.0) as i32, (i / size.0) as i32);
            for d in 0..delta_size.len() {
                let ds = delta_size[d];
                let npos = (pos.0 + ds.0, pos.1 + ds.1);
                //println!("{npos:?}");
                if npos.0 >= 0 && npos.0 < size.0 as i32 && npos.1 >= 0 && npos.1 < size.1 as i32 {
                    //println!("old: {}, new: {}", map[pos.0 as usize + pos.1 as usize * size.0], map[npos.0 as usize + npos.1 as usize * size.0]);
                    if map[npos.0 as usize + npos.1 as usize * size.0] - map[pos.0 as usize + pos.1 as usize * size.0] - 2 >= min_skip {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

fn partB(input: &str) -> u32 {
    let mut map = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut size = (0, 0);
    let mut y = 0;
    for i in input.lines() {
        let mut x = 0;
        for j in i.chars() {
            if j == 'S' {
                start = (x, y);
            }
            if j == 'E' {
                end = (x, y);
            }
            if j == '#' {
                map.push(-1);
            } else {
                map.push(0);
            }
            x+=1;
        }
        size = (x, 0);
        y+=1;
    }
    size = (size.0, y);

    // dfs to fill out map
    let mut stack = vec![start.clone()];
    let mut path = vec![];
    while stack.len() > 0 {
        // check adjacent
        let s = stack.pop().unwrap();
        path.push(s);
        let v = s.0 + s.1 * size.0;
        let new = map[v] + 1;
        if s.0 > 0 {
            if map[v-1] == 0 && (s.0 - 1 != start.0 || s.1 != start.1) {
                stack.push((s.0 - 1, s.1));
                map[v-1] = new;
            }
        }
        if s.0 < size.0 - 1 {
            if map[v+1] == 0 && (s.0 + 1 != start.0 || s.1 != start.1) {
                stack.push((s.0 + 1, s.1));
                map[v+1] = new;
            }
        }
        if s.1 > 0 {
            if map[v-size.0] == 0 && (s.0 != start.0 || s.1 -1  != start.1) {
                stack.push((s.0, s.1 - 1));
                map[v-size.0] = new;
            }
        }
        if s.1 < size.1 - 1 {
            if map[v+size.0] == 0 && (s.0 != start.0 || s.1 + 1 != start.1) {
                stack.push((s.0, s.1 + 1));
                map[v+size.0] = new;
            }
        }
    }
    /*
    let mut delta_size = vec![];
    for dx in -2..=2i32 {
        for dy in -2..=2i32 {
            if dx.abs() + dy.abs() == 2 {
                delta_size.push((dx, dy));
            }
        }
    }
     */
    let mut sum = 0;
    let min_skip = 100;
    let skip_dist = 20;
    // println!("{path:?}, {}", path.len());
    for i in 0..path.len() {
        let p1 = path[i];
        for j in (min_skip+i)..path.len() {
            let p2 = path[j];
            let (dx, dy) = (p2.0 as i32 - p1.0 as i32, p2.1 as i32 - p1.1 as i32);
            // println!("{}", dx.abs() + dy.abs());
            if dx.abs() + dy.abs() <= 20 && dx.abs() + dy.abs() <= j as i32 - i as i32 - min_skip as i32 {
                // println!("{p1:?}, {p2:?}");
                sum += 1;
            }
        }
    }
    sum

}