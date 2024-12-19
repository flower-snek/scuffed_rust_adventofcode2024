use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let testinput = include_str!("input_test.txt");
    println!("Part A TEST: {:?}", partA(testinput, 6, 12));
    println!("Part A REAL: {:?}", partA(input, 70, 1024));
    println!("===============================");
    println!("Part B TEST: {:?}", partB(testinput, 6, 12));
    println!("Part B REAL: {:?}", partB(input, 70, 1024));
}

fn partA(input: &str, size: usize, sim_bytes: usize) -> usize {
    let wh = size + 1;
    let grid_size = wh*wh;
    let mut corrupted = vec![false; grid_size];
    let mut bytes = input.lines();
    for i in 0..sim_bytes {
        let mut b = bytes.next().unwrap().split(",").collect::<Vec<&str>>();
        let x = b[0].parse::<usize>().unwrap();
        let y = b[1].parse::<usize>().unwrap();
        corrupted[x + y * wh] = true;
    }
    // println!("{bytes:?}");
    // bfs wooo
    // x y depth
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::from([(0,0,0)]);
    let mut visited = vec![(0,0)];
    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        // check all directions
        if cur.0 > 0 {
            let new = (cur.0 - 1, cur.1);
            if !visited.contains(&new) && !corrupted[new.0 + new.1 * wh] {
                queue.push_back((new.0, new.1, cur.2+1));
                visited.push(new);
            }
        }
        if cur.1 > 0 {
            let new = (cur.0, cur.1 - 1);
            if !visited.contains(&new) && !corrupted[new.0 + new.1 * wh] {
                queue.push_back((new.0, new.1, cur.2+1));
                visited.push(new);
            }
        }
        if cur.0 < wh - 1 {
            let new = (cur.0 + 1, cur.1);
            if !visited.contains(&new) && !corrupted[new.0 + new.1 * wh] {
                queue.push_back((new.0, new.1, cur.2+1));
                visited.push(new);
            }
        }
        if cur.1 < wh - 1 {
            let new = (cur.0, cur.1 + 1);
            if !visited.contains(&new) && !corrupted[new.0 + new.1 * wh] {
                queue.push_back((new.0, new.1, cur.2+1));
                visited.push(new);
            }
        }
        if cur.0 == size && cur.1 == size {
            return cur.2;
        }
    }
    0
}

fn partB(input: &str, size: usize, sim_bytes: usize) -> (usize, usize) {
    let wh = size + 1;
    let grid_size = wh*wh;
    let mut corrupted = vec![false; grid_size];
    let mut bytes = input.lines();
    let mut bytes_v = vec![];
    for i in bytes {
        let mut b = i.split(",").collect::<Vec<&str>>();
        let x = b[0].parse::<usize>().unwrap();
        let y = b[1].parse::<usize>().unwrap();
        bytes_v.push((x, y));
    }
    for i in 0..sim_bytes {
        corrupted[bytes_v[i].0 + bytes_v[i].1 * wh] = true;
    }
    // println!("{bytes:?}");
    // bfs wooo
    // x y depth
    let mut b = sim_bytes;
    let mut last_flood: Vec<(usize, usize)> = vec![];
    loop {
        corrupted[bytes_v[b].0 + bytes_v[b].1 * wh] = true;
        if last_flood.contains(&bytes_v[b]) || b == sim_bytes {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);
            let mut queue_paths: VecDeque<Vec<(usize, usize)>> = VecDeque::from([vec![]]);
            let mut visited = vec![(0, 0)];
            let mut reachable = false;
            while queue.len() > 0 && !reachable {
                let cur = queue.pop_front().unwrap();
                let mut cur_path = queue_paths.pop_front().unwrap();
                cur_path.push(cur);
                // check all directions
                if cur.0 > 0 {
                    let new = (cur.0 - 1, cur.1);
                    if !visited.contains(&new) && !queue.contains(&new) && !corrupted[new.0 + new.1 * wh] {
                        queue.push_back((new.0, new.1));
                        queue_paths.push_back(cur_path.clone());
                        visited.push(new);
                    }
                }
                if cur.1 > 0 {
                    let new = (cur.0, cur.1 - 1);
                    if !visited.contains(&new) && !queue.contains(&new) && !corrupted[new.0 + new.1 * wh] {
                        queue.push_back((new.0, new.1));
                        queue_paths.push_back(cur_path.clone());
                        visited.push(new);
                    }
                }
                if cur.0 < wh - 1 {
                    let new = (cur.0 + 1, cur.1);
                    if !visited.contains(&new) && !queue.contains(&new) && !corrupted[new.0 + new.1 * wh] {
                        queue.push_back((new.0, new.1));
                        queue_paths.push_back(cur_path.clone());
                        visited.push(new);
                    }
                }
                if cur.1 < wh - 1 {
                    let new = (cur.0, cur.1 + 1);
                    if !visited.contains(&new) && !queue.contains(&new) && !corrupted[new.0 + new.1 * wh] {
                        queue.push_back((new.0, new.1));
                        queue_paths.push_back(cur_path.clone());
                        visited.push(new);
                    }
                }
                if cur.0 == size && cur.1 == size {
                    reachable = true;
                    last_flood = cur_path.clone();
                }
            }
            if !reachable {
                return bytes_v[b];
            }
        }
        b += 1;
        if b % 10 == 0 {
            println!("{b}")
        }
    }
}