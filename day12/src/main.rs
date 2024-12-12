const INPUT: &str = include_str!("input.txt");


// DONT LOOK AT ME THIS CODE SUCKS I NEED TO REFACTOR IT
// DONT LOOK AT ME THIS CODE SUCKS I NEED TO REFACTOR IT
// DONT LOOK AT ME THIS CODE SUCKS I NEED TO REFACTOR IT
// DONT LOOK AT ME THIS CODE SUCKS I NEED TO REFACTOR IT
// DONT LOOK AT ME THIS CODE SUCKS I NEED TO REFACTOR IT
// DONT LOOK AT ME THIS CODE SUCKS I NEED TO REFACTOR IT
// ... see 599 more ...


fn main() {
    let mut garden: Vec<char> = vec![];
    let mut width = 0;
    for i in INPUT.lines() {
        width = i.len();
        for j in i.chars(){
            garden.push(j);
        }
    }
    let mut temp_garden = garden.clone();
    let mut garden_walls:Vec<(bool, bool, bool, bool)> = vec![]; // ldur
    for _ in 0..temp_garden.len(){
        garden_walls.push((false, false, false, false));
    }
    let mut sum = 0;
    let mut sumb = 0;

    for i in 0..temp_garden.len() {
        let j = temp_garden[i];
        if j != '-' {
            // breadth first
            let mut stack: Vec<usize> = vec![i];
            let mut searched: Vec<usize> = vec![];
            let mut area = 0;
            let mut perim = 0;

            while stack.len() > 0 {
                let t = stack.pop().unwrap();
                if !stack.contains(&t) {
                    let (x, y) = ((t % width) as i32, (t / width) as i32);
                    let mut fence_walls = 4;
                    let mut wall_pos:(bool, bool, bool, bool) = (true, true, true, true); // ldur
                    if x > 0 {
                        if temp_garden[t - 1] == j {
                            if !searched.contains(&(t - 1)) {
                                stack.push(t - 1);
                            }
                            fence_walls -= 1;
                            wall_pos.0 = false;
                        }
                    }
                    if x < (width - 1) as i32 {
                        if temp_garden[t + 1] == j {
                            if !searched.contains(&(t + 1)) {
                                stack.push(t + 1);
                            }
                            fence_walls -= 1;
                            wall_pos.3 = false;
                        }
                    }
                    if y > 0 {
                        if temp_garden[t - width] == j {
                            if !searched.contains(&(t - width)) {
                                stack.push(t - width);
                            }
                            fence_walls -= 1;
                            wall_pos.2 = false;
                        }
                    }
                    // assume square bc lazy
                    if y < (width - 1) as i32 {
                        if temp_garden[t + width] == j {
                            if !searched.contains(&(t + width)) {
                                stack.push(t + width);
                            }
                            fence_walls -= 1;
                            wall_pos.1 = false;
                        }
                    }
                    area += 1;
                    perim += fence_walls;
                    searched.push(t);
                    garden_walls[t] = wall_pos;
                }
            }
            //println!("{searched:?}");
            //println!("{j}: {area} {perim}");
            sum += area * perim;

            let mut walls = 0;
            for i in 0..searched.len() {
                let t = searched[i];
                temp_garden[t] = '-';
                // check along each wall in other searched-s
                // i regret using a tuple here
                if garden_walls[t].0 { // left wall; +- y axis
                    let mut y = t / width;
                    let x = t % width;
                    // check up
                    let mut checking = true;
                    while checking && y < width-1{
                        y += 1;
                        let v = x + y * width;
                        if searched.contains(&v){
                            if garden_walls[v].0 {
                                garden_walls[v].0 = false;
                            }else {
                                checking = false;
                            }
                        }else {
                            checking = false;
                        }
                    }
                    // and down
                    y = t/width;
                    checking = true;
                    while checking && y > 0{
                        y -= 1;
                        let v = x + y * width;
                        if searched.contains(&v){
                            if garden_walls[v].0 {
                                garden_walls[v].0 = false;
                            }else {
                                checking = false;
                            }
                        }else {
                            checking = false;
                        }
                    }
                    walls += 1;
                }
                if garden_walls[t].3 {
                    let mut y = t / width;
                    let x = t % width;
                    // check up
                    let mut checking = true;
                    while checking  && y < width-1{
                        y += 1;
                        let v = x + y * width;
                        if searched.contains(&v){
                            if garden_walls[v].3 {
                                garden_walls[v].3 = false;
                            }else {
                                checking = false;
                            }
                        }else {
                            checking = false;
                        }
                    }
                    // and down
                    y = t/width;
                    checking = true;
                    while checking && y > 0{
                        y -= 1;
                        let v = x + y * width;
                        if searched.contains(&v){
                            if garden_walls[v].3 {
                                garden_walls[v].3 = false;
                            }else {
                                checking = false;
                            }
                        }else {
                            checking = false;
                        }
                    }
                    walls += 1;
                }
                if garden_walls[t].2 {
                    let y = t / width;
                    let mut x = t % width;
                    // check right
                    let mut checking = true;
                    while checking && x < width - 1{
                        x += 1;
                        let v = x + y * width;
                        if searched.contains(&v){
                            if garden_walls[v].2 {
                                garden_walls[v].2 = false;
                            }else {
                                checking = false;
                            }
                        }else {
                            checking = false;
                        }
                    }
                    // and left
                    x = t%width;
                    checking = true;
                    while checking && x > 0{
                        x -= 1;
                        let v = x + y * width;
                        if searched.contains(&v){
                            if garden_walls[v].2 {
                                garden_walls[v].2 = false;
                            }else {
                                checking = false;
                            }
                        }else {
                            checking = false;
                        }
                    }
                    walls += 1;
                }
                if garden_walls[t].1 {
                    let y = t / width;
                    let mut x = t % width;
                    // check right
                    let mut checking = true;
                    while checking && x < width - 1{
                        x += 1;
                        let v = x + y * width;
                        if searched.contains(&v){
                            if garden_walls[v].1 {
                                garden_walls[v].1 = false;
                            }else {
                                checking = false;
                            }
                        }else {
                            checking = false;
                        }
                    }
                    // and left
                    x = t%width;
                    checking = true;
                    while checking && x > 0{
                        x -= 1;
                        let v = x + y * width;
                        if searched.contains(&v){
                            if garden_walls[v].1 {
                                garden_walls[v].1 = false;
                            }else {
                                checking = false;
                            }
                        }else {
                            checking = false;
                        }
                    }
                    walls += 1;
                }
            }
            sumb += area * walls;
        }
    }
    println!("Part A: {sum}");
    println!("Part B: {sumb}");
}