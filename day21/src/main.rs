use std::collections::HashMap;

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
    let codes: Vec<_> = input.lines().collect();
    // idea: determine the length of each path on the codes
    let mut button_hash: HashMap<(char, char), String> = HashMap::new(); // for human -> robot1
    let mut robo_hash1: HashMap<(char, char, char), String> = HashMap::new(); // for robot1 -> robot2
    let mut robo_hash2: HashMap<(char, char, char, char), String> = HashMap::new(); // for robot2 -> robot3
    // the tuples here are positions of the outer arms (none in human -> robot case), start char, end char. the output is a string of the path.
    // for each starting position of the robot arm, starting position of the next arm, and end position of the next arm, find the shortest path:
    let arrow_robot: [(char, u64, u64); 5] = [('<', 0, 1), ('^', 1, 0), ('v', 1, 1), ('A', 2, 0), ('>', 2, 1)];
    let keypad_robot: [(char, u64, u64); 11] = [('7', 0, 0), ('8', 1, 0), ('9', 2, 0),
                                                ('4', 0, 1), ('5', 1, 1), ('6', 2, 1),
                                                ('1', 0, 2), ('2', 1, 2), ('3', 2, 2),
                                                             ('0', 1, 3), ('A', 2, 3)];
    fn get_combos(c1: char, c2: char, n1: u64, n2: u64) -> Vec<String> {
        if n1 == 0 {
            if n2 == 0 {
                return vec!["".to_string()];
            }
            let mut lower_combos = get_combos(c1, c2, n1, n2 - 1);
            for l in 0..lower_combos.len() {
                lower_combos[l].push(c2);
            }
            return lower_combos;
        } else if n2 == 0 {
            let mut lower_combos = get_combos(c1, c2, n1 - 1, n2);
            for l in 0..lower_combos.len() {
                lower_combos[l].push(c1);
            }
            return lower_combos;
        } else {
            let mut combo1 = get_combos(c1, c2, n1 - 1, n2);
            for l in 0..combo1.len() {
                combo1[l].push(c1);
            }
            let mut combo2 = get_combos(c1, c2, n1, n2 - 1);
            for l in 0..combo2.len() {
                combo2[l].push(c2);
            }
            combo1.append(&mut combo2);
            return combo1;
        }
    }

    // human -> robot paths (this is just for checking work really)
    for p_s in arrow_robot {
        for p_e in arrow_robot {
            let hash_in = (p_s.0, p_e.0);
            // find all start-to-end paths (since we're on a square grid going backwards will only lose time so this should be easy)
            let mut horiz_char = '>';
            let mut vert_char = 'v';
            let mut dx = 0;
            let mut dy = 0;
            if p_e.1 < p_s.1 {
                horiz_char = '<';
                dx = p_s.1 - p_e.1;
            }else{
                dx = p_e.1 - p_s.1;
            }
            if p_e.2 < p_s.2 {
                vert_char = '^';
                dy = p_s.2 - p_e.2;
            }else{
                dy = p_e.2 - p_s.2
            }
            let combos = get_combos(horiz_char, vert_char, dx, dy);
            let mut valid_combos: Vec<String> = vec![];
            // check combos to make sure none cross over 0,0
            for c in combos {
                let (mut x, mut y) = (p_s.1, p_s.2);
                let mut valid = true;

                for ch in c.chars() {
                    if ch == '<' {
                        x -= 1;
                    }
                    if ch == '>' {
                        x += 1;
                    }
                    if ch == '^' {
                        y -= 1;
                    }
                    if ch == 'v' {
                        y += 1;
                    }
                    if (x, y) == (0, 0) {
                        valid = false;
                    }
                }
                if valid {
                    valid_combos.push(c);
                }
            }
            // just use the first lol
            // println!("{}, {}, {}", p_s.0, p_e.0, valid_combos[0]);
            let mut cl = valid_combos[0].clone();
            cl.push('A');
            button_hash.insert(hash_in, cl);
        }
    }



    for a in arrow_robot {
        // first robot -> second robot
        // in terms of human inputs
        for p_s in arrow_robot {
            for p_e in arrow_robot {
                let hash_in = (a.0, p_s.0, p_e.0);
                // find all start-to-end paths (since we're on a square grid going backwards will only lose time so this should be easy)
                let mut horiz_char = '>';
                let mut vert_char = 'v';
                let mut dx = 0;
                let mut dy = 0;
                if p_e.1 < p_s.1 {
                    horiz_char = '<';
                    dx = p_s.1 - p_e.1;
                }else{
                    dx = p_e.1 - p_s.1;
                }
                if p_e.2 < p_s.2 {
                    vert_char = '^';
                    dy = p_s.2 - p_e.2;
                }else{
                    dy = p_e.2 - p_s.2
                }
                let combos = get_combos(horiz_char, vert_char, dx, dy);
                let mut valid_combos: Vec<String> = vec![];
                let mut best_path: String = "".to_string();
                // check combos to make sure none cross over 0,0 and to find the best
                // println!("{combos:?}");
                for c in combos {
                    let (mut x, mut y) = (p_s.1, p_s.2);
                    let mut valid = true;
                    let mut this_path = "".to_string();
                    let mut cur_pos = a.0;
                    for ch in c.chars() {
                        if ch == '<' {
                            x -= 1;
                        }
                        if ch == '>' {
                            x += 1;
                        }
                        if ch == '^' {
                            y -= 1;
                        }
                        if ch == 'v' {
                            y += 1;
                        }
                        if (x, y) == (0, 0) {
                            valid = false;
                        }else {
                            this_path.push_str(button_hash.get(&(cur_pos, ch)).unwrap());
                            // print!("{}, ", cur_pos);
                            cur_pos = ch;
                            // println!("{}, {}", cur_pos, this_path);
                        }
                    }
                    if valid {
                        valid_combos.push(c);
                        this_path.push_str(button_hash.get(&(cur_pos, 'A')).unwrap());
                        if this_path.len() < best_path.len() || best_path == "".to_string(){
                            best_path = this_path.to_string();
                        }
                    }
                }
                // println!("{}, {}, {}, {}", a.0, p_s.0, p_e.0, best_path);
                robo_hash1.insert(hash_in, best_path);
            }
        }
    }
    for a in arrow_robot {
        // second robot -> third robot
        // in terms of first layer inputs
        for a2 in arrow_robot {
            for p_s in keypad_robot {
                for p_e in keypad_robot {
                    let hash_in = (a.0, a2.0, p_s.0, p_e.0);
                    // find all start-to-end paths (since we're on a square grid going backwards will only lose time so this should be easy)
                    let mut horiz_char = '>';
                    let mut vert_char = 'v';
                    let mut dx = 0;
                    let mut dy = 0;
                    if p_e.1 < p_s.1 {
                        horiz_char = '<';
                        dx = p_s.1 - p_e.1;
                    }else{
                        dx = p_e.1 - p_s.1;
                    }
                    if p_e.2 < p_s.2 {
                        vert_char = '^';
                        dy = p_s.2 - p_e.2;
                    }else{
                        dy = p_e.2 - p_s.2
                    }
                    let combos = get_combos(horiz_char, vert_char, dx, dy);
                    let mut valid_combos: Vec<String> = vec![];
                    let mut best_path: String = "".to_string();
                    // check combos to make sure none cross over 0,0 and to find the best
                    // println!("{combos:?}");
                    for c in combos {
                        let (mut x, mut y) = (p_s.1, p_s.2);
                        let mut valid = true;
                        let mut this_path = "".to_string();
                        let mut cur_pos_outer = a.0;
                        let mut cur_pos_inner = a2.0;
                        for ch in c.chars() {
                            if ch == '<' {
                                x -= 1;
                            }
                            if ch == '>' {
                                x += 1;
                            }
                            if ch == '^' {
                                y -= 1;
                            }
                            if ch == 'v' {
                                y += 1;
                            }
                            if (x, y) == (0, 3) {
                                valid = false;
                            }else {
                                this_path.push_str(robo_hash1.get(&(cur_pos_outer, cur_pos_inner, ch)).unwrap());
                                // print!("{}, ", cur_pos);
                                cur_pos_outer = 'A';
                                cur_pos_inner = ch;
                                // println!("{}, {}", cur_pos, this_path);
                            }
                        }
                        if valid {
                            valid_combos.push(c);
                            this_path.push_str(robo_hash1.get(&(cur_pos_outer, cur_pos_inner, 'A')).unwrap());
                            if this_path.len() < best_path.len() || best_path == "".to_string(){
                                best_path = this_path.to_string();
                            }
                        }
                    }
                    // println!("{}, {}, {}, {}, {}", a.0, a2.0, p_s.0, p_e.0, best_path);
                    robo_hash2.insert((a.0, a2.0, p_s.0, p_e.0), best_path);
                }
            }
        }
    }
    let mut sum = 0;
    for c in codes {
        let mut path = "".to_string();
        let mut prev_char = 'A';
        for ch in c.chars() {
            path.push_str(robo_hash2.get(&('A', 'A', prev_char, ch)).unwrap());
            prev_char = ch;
        }
        // println!("{c}, {path}, {}", path.len());
        let complexity: usize = path.len() * c[..3].parse::<usize>().unwrap();
        // println!("{complexity}");
        sum += complexity;
    }
    sum as u32
}

fn partB(input: &str) -> u64 {
    let codes: Vec<_> = input.lines().collect();
    // idea: determine the length of each path on the codes
    let rc_layers = 25;
    let mut button_hash: HashMap<(char, char, usize), u64> = HashMap::new(); // for human -> robot N
    let mut keypad_hash: HashMap<(char, char), u64> = HashMap::new(); // for last robots
    let arrow_robot: [(char, u64, u64); 5] = [('<', 0, 1), ('^', 1, 0), ('v', 1, 1), ('A', 2, 0), ('>', 2, 1)];
    let keypad_robot: [(char, u64, u64); 11] = [('7', 0, 0), ('8', 1, 0), ('9', 2, 0),
        ('4', 0, 1), ('5', 1, 1), ('6', 2, 1),
        ('1', 0, 2), ('2', 1, 2), ('3', 2, 2),
        ('0', 1, 3), ('A', 2, 3)];
    fn get_combos(c1: char, c2: char, n1: u64, n2: u64) -> Vec<String> {
        if n1 == 0 {
            if n2 == 0 {
                return vec!["".to_string()];
            }
            let mut lower_combos = get_combos(c1, c2, n1, n2 - 1);
            for l in 0..lower_combos.len() {
                lower_combos[l].push(c2);
            }
            return lower_combos;
        } else if n2 == 0 {
            let mut lower_combos = get_combos(c1, c2, n1 - 1, n2);
            for l in 0..lower_combos.len() {
                lower_combos[l].push(c1);
            }
            return lower_combos;
        } else {
            let mut combo1 = get_combos(c1, c2, n1 - 1, n2);
            for l in 0..combo1.len() {
                combo1[l].push(c1);
            }
            let mut combo2 = get_combos(c1, c2, n1, n2 - 1);
            for l in 0..combo2.len() {
                combo2[l].push(c2);
            }
            combo1.append(&mut combo2);
            return combo1;
        }
    }

    // human -> robot paths (this is just for checking work really)
    for p_s in arrow_robot {
        for p_e in arrow_robot {
            let hash_in = (p_s.0, p_e.0, 0);
            // find all start-to-end paths (since we're on a square grid going backwards will only lose time so this should be easy)
            let mut horiz_char = '>';
            let mut vert_char = 'v';
            let mut dx = 0;
            let mut dy = 0;
            if p_e.1 < p_s.1 {
                horiz_char = '<';
                dx = p_s.1 - p_e.1;
            }else{
                dx = p_e.1 - p_s.1;
            }
            if p_e.2 < p_s.2 {
                vert_char = '^';
                dy = p_s.2 - p_e.2;
            }else{
                dy = p_e.2 - p_s.2
            }
            let combos = get_combos(horiz_char, vert_char, dx, dy);
            let mut valid_combos: Vec<String> = vec![];
            // check combos to make sure none cross over 0,0
            for c in combos {
                let (mut x, mut y) = (p_s.1, p_s.2);
                let mut valid = true;

                for ch in c.chars() {
                    if ch == '<' {
                        x -= 1;
                    }
                    if ch == '>' {
                        x += 1;
                    }
                    if ch == '^' {
                        y -= 1;
                    }
                    if ch == 'v' {
                        y += 1;
                    }
                    if (x, y) == (0, 0) {
                        valid = false;
                    }
                }
                if valid {
                    valid_combos.push(c);
                }
            }
            // just use the first lol
            // println!("{}, {}, {}", p_s.0, p_e.0, valid_combos[0]);
            let mut cl = valid_combos[0].clone();
            cl.push('A');
            button_hash.insert(hash_in, cl.len() as u64);
        }
    }
    for i in 1..rc_layers {
        for p_s in arrow_robot {
            for p_e in arrow_robot {
                let hash_in = (p_s.0, p_e.0, i);
                // find all start-to-end paths (since we're on a square grid going backwards will only lose time so this should be easy)
                let mut horiz_char = '>';
                let mut vert_char = 'v';
                let mut dx = 0;
                let mut dy = 0;
                if p_e.1 < p_s.1 {
                    horiz_char = '<';
                    dx = p_s.1 - p_e.1;
                }else{
                    dx = p_e.1 - p_s.1;
                }
                if p_e.2 < p_s.2 {
                    vert_char = '^';
                    dy = p_s.2 - p_e.2;
                }else{
                    dy = p_e.2 - p_s.2
                }
                let combos = get_combos(horiz_char, vert_char, dx, dy);
                let mut valid_combos: Vec<String> = vec![];
                let mut best_path = 0;
                // check combos to make sure none cross over 0,0
                for c in combos {
                    let (mut x, mut y) = (p_s.1, p_s.2);
                    let mut valid = true;
                    let mut this_path = 0;
                    let mut cur_pos = 'A';
                    for ch in c.chars() {
                        if ch == '<' {
                            x -= 1;
                        }
                        if ch == '>' {
                            x += 1;
                        }
                        if ch == '^' {
                            y -= 1;
                        }
                        if ch == 'v' {
                            y += 1;
                        }
                        if (x, y) == (0, 0) {
                            valid = false;
                        } else {
                            this_path += button_hash.get(&(cur_pos, ch, i - 1)).unwrap();
                            cur_pos = ch;
                        }
                    }
                    if valid {
                        valid_combos.push(c);
                        this_path += button_hash.get(&(cur_pos, 'A', i - 1)).unwrap();
                        if this_path < best_path || best_path == 0{
                            best_path = this_path;
                        }
                    }
                }
                // println!("{}, {}, {}, {best_path}", p_s.0, p_e.0, i);
                button_hash.insert(hash_in, best_path);
            }
        }
    }
    for p_s in keypad_robot {
        for p_e in keypad_robot {
            let hash_in = (p_s.0, p_e.0);
            // find all start-to-end paths (since we're on a square grid going backwards will only lose time so this should be easy)
            let mut horiz_char = '>';
            let mut vert_char = 'v';
            let mut dx = 0;
            let mut dy = 0;
            if p_e.1 < p_s.1 {
                horiz_char = '<';
                dx = p_s.1 - p_e.1;
            }else{
                dx = p_e.1 - p_s.1;
            }
            if p_e.2 < p_s.2 {
                vert_char = '^';
                dy = p_s.2 - p_e.2;
            }else{
                dy = p_e.2 - p_s.2
            }
            let combos = get_combos(horiz_char, vert_char, dx, dy);
            let mut valid_combos: Vec<String> = vec![];
            let mut best_path = 0;
            // check combos to make sure none cross over 0,0 and to find the best
            // println!("{combos:?}");
            for c in combos {
                let (mut x, mut y) = (p_s.1, p_s.2);
                let mut valid = true;
                let mut this_path = 0;
                let mut cur_pos = 'A';
                for ch in c.chars() {
                    if ch == '<' {
                        x -= 1;
                    }
                    if ch == '>' {
                        x += 1;
                    }
                    if ch == '^' {
                        y -= 1;
                    }
                    if ch == 'v' {
                        y += 1;
                    }
                    if (x, y) == (0, 3) {
                        valid = false;
                    }else {
                        this_path += button_hash.get(&(cur_pos, ch, rc_layers-1)).unwrap();
                        // print!("{}, ", cur_pos);
                        cur_pos = ch;
                        // println!("{}, {}", cur_pos, this_path);
                    }
                }
                if valid {
                    valid_combos.push(c);
                    this_path += button_hash.get(&(cur_pos, 'A', rc_layers-1)).unwrap();
                    if this_path < best_path || best_path == 0{
                        best_path = this_path;
                    }
                }
            }
            // println!("{}, {}, {}, {}, {}", a.0, a2.0, p_s.0, p_e.0, best_path);
            // println!("{}, {}, {best_path}", p_s.0, p_e.0);
            keypad_hash.insert(hash_in, best_path);
        }
    }
    let mut sum = 0;
    for c in codes {
        let mut path = 0;
        let mut prev_char = 'A';
        for ch in c.chars() {
            path += keypad_hash.get(&(prev_char, ch)).unwrap();
            prev_char = ch;
        }
        // println!("{c}, {path}, {}", path.len());
        let complexity: u64 = path * c[..3].parse::<u64>().unwrap();
        // println!("{complexity}");
        sum += complexity;
    }
    sum as u64
}