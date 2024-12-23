use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let testinput = include_str!("input_test.txt");
    println!("Part A TEST: {:?}", part_a(testinput));
    println!("Part A REAL: {:?}", part_a(input));
    println!("===============================");
    println!("Part B TEST: {:?}", part_b(testinput));
    println!("Part B REAL: {:?}", part_b(input));
}

fn part_a(input: &str) -> u64 {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for i in input.lines() {
        let (c1, c2) = i.split_once("-").unwrap();
        let c1_con = connections.entry(c1).or_insert(vec![]);
        (*c1_con).push(c2);
        let c2_con = connections.entry(c2).or_insert(vec![]);
        (*c2_con).push(c1);
    }
    let mut sum = 0;
    let mut valid_triples: Vec<(&str, &str, &str)> = vec![];
    let mut checked_con: Vec<&str> = vec![];
    for (&c, v) in &connections {
        // println!("{c}, {v:?}");
        for i2 in 0..v.len() {
            let c2 = *v.get(i2).unwrap();
            for i3 in i2..v.len() {
                let c3 = *v.get(i3).unwrap();
                if !checked_con.contains(&c2) && !checked_con.contains(&c3) && connections.get(c2).unwrap().contains(&c3) {
                    valid_triples.push((c, c2, c3));
                }
            }
        }
        checked_con.push(c);
    }
    for t in valid_triples {
        if t.0.chars().next().unwrap() == 't' ||
            t.1.chars().next().unwrap() == 't' ||
            t.2.chars().next().unwrap() == 't' {
            // println!("{t:?}");
            sum += 1;
        }
    }
    sum
}

fn part_b(input: &str) -> String {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for i in input.lines() {
        let (c1, c2) = i.split_once("-").unwrap();
        let c1_con = connections.entry(c1).or_insert(vec![]);
        (*c1_con).push(c2);
        let c2_con = connections.entry(c2).or_insert(vec![]);
        (*c2_con).push(c1);
    }
    let mut webs: Vec<Vec<&str>> = vec![];
    for &c in connections.keys() {
        // println!("{}", c);
        let mut stack: Vec<(&str, Vec<&str>)> = vec![(c, vec![c])];
        while stack.len() > 0 {
            // println!("{}", stack.len());
            let t_con = stack.pop().unwrap();
            let t_con_con = connections.get(t_con.0).unwrap();
            //println!("== {:?} ==", t_con.1);
            for &tc2 in t_con_con {
                //println!("{tc2}");
                if !t_con.1.contains(&tc2) {
                    //println!(" {tc2}");
                    let mut valid = true;
                    // let tc2_con = connections.get(tc2).unwrap();
                    for i in 0..t_con.1.len() {
                        if !connections.get(t_con.1.get(i).unwrap()).unwrap().contains(&tc2) {
                            //println!("  {tc2} doesnt contain {}", t_con.1.get(i).unwrap());
                            valid = false;
                        }
                    }
                    if valid {
                        let mut tc1_new = t_con.1.clone();
                        tc1_new.push(tc2);
                        let mut sub_of_web = false;
                        for wi in 0..webs.len() {
                            let w = webs.get(wi).unwrap();
                            let mut overlap = true;
                            for tci in 0..tc1_new.len() {
                                if !w.contains(tc1_new.get(tci).unwrap()){
                                    overlap = false;
                                }
                            }
                            if overlap {
                                sub_of_web = true;
                            }
                        }
                        if !sub_of_web {
                            stack.push((tc2, tc1_new.clone()));
                            for wi in (0..webs.len()).rev() {
                                let w = webs.get(wi).unwrap();
                                let mut overlap = true;
                                for wi2 in 0..w.len() {
                                    if !tc1_new.contains(w.get(wi2).unwrap()){
                                        overlap = false;
                                    }
                                }
                                if overlap {
                                    webs.remove(wi);
                                }
                            }
                            webs.push(tc1_new)
                        }
                    }
                }
            }
        }
    }
    let mut biggest_party: Vec<&str> = vec![];
    for w in webs {
        if w.len() > biggest_party.len() {
            biggest_party = w;
        }
    }
    biggest_party.sort();
    biggest_party.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",")
}