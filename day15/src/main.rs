

fn main() {
    let real: &str = include_str!("input.txt");
    let test: &str = include_str!("input_test.txt");

    println!("Part A TEST: {}", PartA(test));
    println!("Part B TEST: {}", PartB(test));
    println!("=======================");
    println!("Part A REAL: {}", PartA(real));
    println!("Part B REAL: {}", PartB(real));
}

fn PartA(input: &str) -> usize {
    let mut mode = 0;
    let mut w = 0;
    let mut map = vec![];
    let mut moves: Vec<char> = vec![];
    let mut pos = (0, 0);
    let mut y = 0;
    for i in input.lines() {
        if i.len() == 0 {
            mode = 1;
        }else if mode == 0 {
            // println!("{i}");
            let mut x = 0;
            for j in i.chars() {
                if j == '#' {
                    map.push(2);
                } else if j == 'O' {
                    map.push(1);
                } else if j == '@' {
                    map.push(3);
                    pos = (x, y)
                } else {
                    map.push(0);
                }

                x += 1;
            }
            w = i.len();
            y += 1;
        }else {
            // println!("{i}");
            for j in i.chars() {
                moves.push(j);
            }
        }
    }
    // 2 = wall, 1 = box

    for m in moves {
        let (mut dx, mut dy) = (0i32, 0i32);
        if m == '<' {
            dx = -1;
        }
        if m == '^' {
            dy = -1;
        }
        if m == '>' {
            dx = 1;
        }
        if m == 'v' {
            dy = 1;
        }
        // attempt a move with all objects
        let mut move_stack = vec![pos];
        let mut repeat = true;
        while repeat {
            let &cur = move_stack.last().unwrap();
            let map_next = (cur.0 + dx) as usize + (cur.1 + dy) as usize * w;
            if map[map_next] == 0 {
                repeat = false;
            } else if map[map_next] == 1 {
                move_stack.push((cur.0 + dx, cur.1 + dy));
            } else if map[map_next] == 2 {
                repeat = false;
                move_stack = vec![];
            } else {
                panic!("map[map_next] is value other than 0/1/2");
            }
        }
        while move_stack.len() > 0 {
            let cur = move_stack.pop().unwrap();
            let map_next = (cur.0 + dx) as usize + (cur.1 + dy) as usize * w;
            map[map_next] = map[cur.0 as usize + cur.1 as usize * w];
            map[cur.0 as usize + cur.1 as usize * w] = 0;
            if map[map_next] == 3 {
                pos = (cur.0 + dx, cur.1 + dy);
            }
        }
    }
    let mut sum = 0;
    for j in 0..map.len() {
        let i = map[j];
        if i == 2 {
            //print!("#");
        } else if i == 1 {
            //print!("O");
            sum += (j%w) + 100*(j/w);
        } else {
            //print!(".");
        }
        //if j % w == w - 1 {print!("\n")};
    }
    sum
}

fn PartB(input: &str) -> usize {
    let mut mode = 0;
    let mut w = 0;
    let mut map = vec![];
    let mut moves: Vec<char> = vec![];
    let mut pos = (0, 0);
    let mut y = 0;
    for i in input.lines() {
        if i.len() == 0 {
            mode = 1;
        }else if mode == 0 {
            // println!("{i}");
            let mut x = 0;
            for j in i.chars() {
                if j == '#' {
                    map.push(1);
                    map.push(1);
                } else if j == 'O' {
                    map.push(3);
                    map.push(4);
                } else if j == '@' {
                    map.push(2);
                    map.push(0);
                    pos = (x*2, y)
                } else {
                    map.push(0);
                    map.push(0);
                }

                x += 1;
            }
            w = i.len() * 2;
            y += 1;
        }else {
            // println!("{i}");
            for j in i.chars() {
                moves.push(j);
            }
        }
    }
    // 2 = wall, 1 = box

    for &m in moves.iter() {
        let (mut dx, mut dy) = (0i32, 0i32);
        if m == '<' {
            dx = -1;
        }
        if m == '^' {
            dy = -1;
        }
        if m == '>' {
            dx = 1;
        }
        if m == 'v' {
            dy = 1;
        }
        // attempt a move with all objects
        let mut move_stack = vec![pos];
        let mut n = 0;
        while n < move_stack.len() {
            // println!("{}", move_stack.len());
            let &cur = move_stack.get(n).unwrap();
            let map_next = (cur.0 + dx) as usize + (cur.1 + dy) as usize * w;
            // println!("{cur:?}, {}", map[map_next]);
            if map[map_next] == 1 {
                move_stack = vec![];
            } else if map[map_next] == 3 {
                let m1 = (cur.0 + dx, cur.1 + dy);
                let mut b1 = true;
                let m2 = (cur.0 + dx + 1, cur.1 + dy);
                let mut b2 = true;
                if dx == 1 && dy == 0 {
                    b2 = false;
                }
                for i in 0..move_stack.len() {
                    if move_stack[i].0 == m1.0 && move_stack[i].1 == m1.1 {
                        b1 = false;
                    }
                    if move_stack[i].0 == m2.0 && move_stack[i].1 == m2.1 {
                        b2 = false;
                    }
                }
                if b1 {
                    move_stack.push(m1);
                }
                if b2 {
                    move_stack.push(m2);
                }
            } else if map[map_next] == 4 {
                let m1 = (cur.0 + dx, cur.1 + dy);
                let mut b1 = true;
                let m2 = (cur.0 + dx - 1, cur.1 + dy);
                let mut b2 = true;
                if dx == -1 && dy == 0 {
                    b2 = false;
                }
                for i in 0..move_stack.len() {
                    if move_stack[i].0 == m1.0 && move_stack[i].1 == m1.1 {
                        b1 = false;
                    }
                    if move_stack[i].0 == m2.0 && move_stack[i].1 == m2.1 {
                        b2 = false;
                    }
                }
                if b1 {
                    move_stack.push(m1);
                }
                if b2 {
                    move_stack.push(m2);
                }
            } else if map[map_next] != 0 {
                panic!("map[map_next] is value other than 0/1/2/3/4");
            }
            n += 1;
        }
        // println!("{move_stack:?}");
        while move_stack.len() > 0 {
            let cur = move_stack.pop().unwrap();
            let map_next = (cur.0 + dx) as usize + (cur.1 + dy) as usize * w;
            map[map_next] = map[cur.0 as usize + cur.1 as usize * w];
            map[cur.0 as usize + cur.1 as usize * w] = 0;
            if map[map_next] == 2 {
                pos = (cur.0 + dx, cur.1 + dy);
            }
        }
        // for j in 0..map.len() {
        //     let i = map[j];
        //     if i == 2 {
        //         print!("@");
        //     } else if i == 1 {
        //         print!("#");
        //     }else if i == 3 {
        //         print!("[");
        //         // sum += (j % w) + 100 * (j / w);
        //     } else if i == 4 {
        //         print!("]");
        //     } else {
        //         print!(".");
        //     }
        //     if j % w == w - 1 {print!("\n")};
        // }
    }
    let mut sum = 0;
    for j in 0..map.len() {
        let i = map[j];
        if i == 2 {
            print!("@");
        } else if i == 1 {
            print!("#");
        }else if i == 3 {
            print!("[");
            sum += (j % w) + 100 * (j / w);
        } else if i == 4 {
            print!("]");
        } else {
            print!(".");
        }
        if j % w == w - 1 {print!("\n")};
    }
    sum
}