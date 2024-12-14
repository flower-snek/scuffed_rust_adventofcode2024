const TINPUT: &str = include_str!("input_test.txt");
const RINPUT: &str = include_str!("input_real.txt");

fn main() {
    println!("=== TEST ===");
    println!("Part A: {}", partA(TINPUT, 11, 7));
    // println!("Part B: {}", partB(TINPUT, 11, 7));

    println!("=== REAL ===");
    println!("Part A: {}", partA(RINPUT, 101, 103));
    // ??
    // println!("Part B: {}", partB(RINPUT, 101, 103));
    println!("Part B is commented out bc it's just a visualizer lol");
}

fn partA(input: &str, w: i32, h: i32) -> i64 {
    let time = 100;
    let mut robots: Vec<Robot> = vec![];
    for i in input.lines() {
        let (p, v) = i[2..].split_once(" v=").unwrap();
        let (px, py) = p.split_once(",").unwrap();
        let (vx, vy) = v.split_once(",").unwrap();
        let r = Robot {
            pos: (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()),
            vel: (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap())
        };
        // println!("{:?} {:?}", r.pos, r.vel);
        robots.push(r);
    }
    let mut quadrants = vec![0, 0,
                                     0, 0];
    for mut r in robots {
        let mut pos = r.pos;
        pos.0 = (((pos.0 + (r.vel.0 * time)) % w) + w) % w;
        pos.1 = (((pos.1 + (r.vel.1 * time)) % h) + h) % h;
        r.set_pos(pos);
        let mut quad = 0;
        if pos.0 >= w - w/2 {
            quad += 1;
        }
        if pos.1 >= h - h/2 {
            quad += 2;
        }

        // println!("{pos:?}");
        if pos.0 != w/2 && pos.1 != h/2 {
            quadrants[quad] += 1;
        }
    }
    // println!("{quadrants:?}");
    quadrants.iter().product()
}

fn partB(input: &str, w: i32, h: i32) -> i64 {
    let mut robots: Vec<Robot> = vec![];
    let mut nr = 0;
    for i in input.lines() {
        let (p, v) = i[2..].split_once(" v=").unwrap();
        let (px, py) = p.split_once(",").unwrap();
        let (vx, vy) = v.split_once(",").unwrap();
        let r = Robot {
            pos: (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()),
            vel: (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap())
        };
        // println!("{:?} {:?}", r.pos, r.vel);
        robots.push(r);
        nr += 1;
    }
    for y in 1..10000 {
        let mut interesting = false;
        let mut map: Vec<i32> = vec![];
        for _ in 0..w*h {
            map.push(0);
        }
        for i in 0..nr {
            let r = robots.get(i).unwrap();
            let mut pos = r.pos;
            pos.0 = (((pos.0 + r.vel.0) % w) + w) % w;
            pos.1 = (((pos.1 + r.vel.1) % h) + h) % h;
            let new_r = Robot {
                pos: pos,
                vel: r.vel
            };
            robots[i] = new_r;

            map[(pos.0 + pos.1 * w) as usize] += 1;
        }

        for i in 0..(w*h) {
            if y > 6000 && i < w*h - 15 {
                let mut int = true;
                for j in 0..6 {
                    if map[(i + j) as usize] == 0 {
                        int = false;
                    }
                }
                interesting = interesting || int;
            }
        // if map[i as usize] > 0 && map[((i + w  +1) % (w*h)) as usize] > 0 && map[((i + w*2 + 2) % (w*h)) as usize] > 0 && map[((i + w*3 + 3) % (w*h)) as usize] > 0 {
            //     interesting = true;
            // }
        }
        if interesting {
            println!("{y}");
            for i in 0..w*h {
                if map[i as usize] == 0 {
                    print!(".");
                }else {
                    print!("{}", map[i as usize]);
                }
                if i % w == w-1 {
                    print!("\n");
                }
            }
        }
    }
    0
}

struct Robot {
    pos: (i32, i32),
    vel: (i32, i32)
}

impl Robot {
    fn set_pos(&mut self, pos: (i32, i32)) {
        self.pos = pos;
    }
}