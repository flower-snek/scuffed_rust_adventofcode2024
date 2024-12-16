
fn main() {
    let input = include_str!("input.txt");
    let testinput = include_str!("input_test.txt");
    println!("Part A TEST: {}", partA(testinput));
    println!("Part A REAL: {}", partA(input));
    println!("===============================");
    println!("Part B TEST: {}", partB(testinput));
    println!("Part B REAL: {}", partB(input));
}

// UNOPTIMIZED DFS GO -->

fn partA(input: &str) -> usize{
    struct Reindeer {
        dir: usize,
        x: usize,
        y: usize,
        score: usize
    }
    let mut map: Vec<bool> = vec![];
    let mut pos = (0, 0);
    let mut end = (0, 0);
    let mut w = 0;
    for i in input.lines() {
        let mut x = 0;
        for j in i.chars(){
            if j == '#' {
                map.push(false);
            }else{
                map.push(true);
                if j == 'S' {
                    pos = (x, w);
                }
                if j == 'E' {
                    end = (x, w);
                }
            }
            x += 1;
        }
        w += 1;
    }
    // dir: 0
    //    3   1
    //      2
    let starting_reindeer = Reindeer {
        dir: 1,
        x: pos.0,
        y: pos.1,
        score: 0
    };
    let mut min_score: Vec<usize> = vec![99999999; w*w];
    let mut reindeer_queue: Vec<Reindeer> = vec![starting_reindeer];
    while reindeer_queue.len() != 0 {
        let t_rein = reindeer_queue.pop().unwrap();
        let t_n = t_rein.x + t_rein.y * w;
        let (mut dx, mut dy): (i32, i32) = (0, 0);
        if t_rein.dir == 0 {
            dy = -1;
        }
        if t_rein.dir == 1 {
            dx = 1;
        }
        if t_rein.dir == 2 {
            dy = 1;
        }
        if t_rein.dir == 3 {
            dx = -1;
        }
        // check forward
        let mut n_n = (t_n as i32 + (dx + dy * (w as i32))) as usize;
        //println!("{}", map[n_n]);
        if map[n_n] {
            if t_rein.score + 1 < min_score[n_n] {
                min_score[n_n] = t_rein.score + 1;
                reindeer_queue.push(Reindeer{
                    dir: t_rein.dir,
                    x: (t_rein.x as i32 + dx) as usize,
                    y: (t_rein.y as i32 + dy) as usize,
                    score: t_rein.score + 1
                });
            }
        }
        // check left
        (dx, dy) = (dy, -dx);
        n_n = (t_n as i32 + (dx + dy * (w as i32))) as usize;
        //println!("{}", map[n_n]);
        if map[n_n] {
            if t_rein.score + 1 < min_score[n_n] {
                min_score[n_n] = t_rein.score + 1001;
                reindeer_queue.push(Reindeer{
                    dir: (t_rein.dir + 3) % 4,
                    x: (t_rein.x as i32 + dx) as usize,
                    y: (t_rein.y as i32 + dy) as usize,
                    score: t_rein.score + 1001
                });
            }
        }
        // check right
        (dx, dy) = (-dx, -dy);
        n_n = (t_n as i32 + (dx + dy * (w as i32))) as usize;
        //println!("{dx}, {dy}");
        //println!("{}", map[n_n]);
        if map[n_n] {
            if t_rein.score + 1 < min_score[n_n] {
                min_score[n_n] = t_rein.score + 1001;
                reindeer_queue.push(Reindeer{
                    dir: (t_rein.dir + 1) % 4,
                    x: (t_rein.x as i32 + dx) as usize,
                    y: (t_rein.y as i32 + dy) as usize,
                    score: t_rein.score + 1001
                });
            }
        }
        /*
        for i in 0..min_score.len() {
            print!("{:05}", min_score[i]);
            if i % w == w-1 {
                print!("\n");
            }
        }
        */
    }
    min_score[end.0 + end.1 * w]
}


fn partB(input: &str) -> usize{
    struct Reindeer {
        dir: usize,
        x: usize,
        y: usize,
        score: usize,
        prev: Vec<usize>
    }
    let mut map: Vec<bool> = vec![];
    let mut pos = (0, 0);
    let mut end = (0, 0);
    let mut w = 0;
    for i in input.lines() {
        let mut x = 0;
        for j in i.chars(){
            if j == '#' {
                map.push(false);
            }else{
                map.push(true);
                if j == 'S' {
                    pos = (x, w);
                }
                if j == 'E' {
                    end = (x, w);
                }
            }
            x += 1;
        }
        w += 1;
    }
    // dir: 0
    //    3   1
    //      2
    let starting_reindeer = Reindeer {
        dir: 1,
        x: pos.0,
        y: pos.1,
        score: 0,
        prev: vec![]
    };
    let mut min_score: Vec<usize> = vec![99999999; w*w];
    let mut optimal_score = 999999999;
    let mut optimal_seats: Vec<usize> = vec![];
    let mut reindeer_queue: Vec<Reindeer> = vec![starting_reindeer];
    while reindeer_queue.len() != 0 {
        let t_rein = reindeer_queue.pop().unwrap();
        let t_n = t_rein.x + t_rein.y * w;
        let (mut dx, mut dy): (i32, i32) = (0, 0);
        if t_rein.dir == 0 {
            dy = -1;
        }
        if t_rein.dir == 1 {
            dx = 1;
        }
        if t_rein.dir == 2 {
            dy = 1;
        }
        if t_rein.dir == 3 {
            dx = -1;
        }
        let mut n_prev = t_rein.prev.clone();
        // check forward
        let mut n_n = (t_n as i32 + (dx + dy * (w as i32))) as usize;
        n_prev.push(t_n);
        if t_rein.x == end.0 && t_rein.y == end.1 {
            if t_rein.score < optimal_score {
                optimal_score = t_rein.score;
                optimal_seats = vec![];
            }
            if t_rein.score <= optimal_score {
                for i in 0..n_prev.len() {
                    if !optimal_seats.contains(&n_prev[i]) {
                        optimal_seats.push(n_prev[i]);
                    }
                }
            }
        }else {
            //println!("{}", map[n_n]);
            if map[n_n] {
                if t_rein.score + 1 <= min_score[n_n] {
                    min_score[n_n] = t_rein.score + 1;
                }
                if t_rein.score + 1 <= min_score[n_n] + 1001 {
                    reindeer_queue.push(Reindeer {
                        dir: t_rein.dir,
                        x: (t_rein.x as i32 + dx) as usize,
                        y: (t_rein.y as i32 + dy) as usize,
                        score: t_rein.score + 1,
                        prev: n_prev.clone()
                    });
                }
            }
            // check left
            (dx, dy) = (dy, -dx);
            n_n = (t_n as i32 + (dx + dy * (w as i32))) as usize;
            //println!("{}", map[n_n]);
            if map[n_n] {
                if t_rein.score + 1001 <= min_score[n_n] {
                    min_score[n_n] = t_rein.score + 1001;
                }
                if t_rein.score + 1001 <= min_score[n_n] + 1001 {
                    reindeer_queue.push(Reindeer {
                        dir: (t_rein.dir + 3) % 4,
                        x: (t_rein.x as i32 + dx) as usize,
                        y: (t_rein.y as i32 + dy) as usize,
                        score: t_rein.score + 1001,
                        prev: n_prev.clone()
                    });
                }
            }
            // check right
            (dx, dy) = (-dx, -dy);
            n_n = (t_n as i32 + (dx + dy * (w as i32))) as usize;
            //println!("{dx}, {dy}");
            //println!("{}", map[n_n]);
            if map[n_n] {
                if t_rein.score + 1001 <= min_score[n_n] {
                    min_score[n_n] = t_rein.score + 1001;
                }
                if t_rein.score + 1001 <= min_score[n_n] + 1001 {
                    reindeer_queue.push(Reindeer {
                        dir: (t_rein.dir + 1) % 4,
                        x: (t_rein.x as i32 + dx) as usize,
                        y: (t_rein.y as i32 + dy) as usize,
                        score: t_rein.score + 1001,
                        prev: n_prev.clone()
                    });
                }
            }
            // println!("{}", reindeer_queue.len())
            /*
            for i in 0..min_score.len() {
                print!("{:05}", min_score[i]);
                if i % w == w-1 {
                    print!("\n");
                }
            }
            */
        }
    }
    optimal_seats.len()
}