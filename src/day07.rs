use std::collections::HashMap;

#[derive(Debug,PartialEq)]
struct Program<'a> {
    weight: u32,
    total_weight: u32,
    subprogs: Vec<&'a str>,
}

fn parse_input(s: &str) -> HashMap<&str, Program> {
    s.lines().map(|line| {
        let mut s = line.split(" -> ");
        let base = s.next().unwrap();
        let mut base = base.split(" (");
        let name = base.next().unwrap();
        let weight = base.next().unwrap();
        let weight = weight[..weight.len()-1].parse().unwrap();
        let subprogs = if let Some(x_sup) = s.next() {
            x_sup.split(", ").collect()
        } else {
            vec![]
        };
        (name, Program { weight, total_weight: 0, subprogs })
        }).collect()
}

fn find_total_weight(progs: &mut HashMap<&str, Program>, name: &str) -> u32 {
    let subprogs = progs[name].subprogs.clone();
    let subprogs_weight: u32 = subprogs.iter()
        .map(|n| find_total_weight(progs, n))
        .sum();
    if let Some(prog) = progs.get_mut(name) {
        prog.total_weight = prog.weight + subprogs_weight;
        prog.total_weight
    } else {
        0
    }
}

// return None if balanced, or Some((sub_prog_name,k)) if unbalanced
// where k is required size
fn check_subs<'a>(progs: &'a HashMap<&'a str, Program>, name: &'a str) -> Option<(&'a str,u32)> {
    let prog = &progs[name];

    // check for leaf -- balanced by definition
    let num_subs = prog.subprogs.len() as u32;
    if num_subs == 0 {
        return None;
    }

    let sub_weights: Vec<u32> = prog.subprogs.iter().map(|&p| progs[p].total_weight).collect();
    let total: u32 = sub_weights.iter().sum();
    let min: u32 = *sub_weights.iter().min().unwrap();
    let max: u32 = *sub_weights.iter().max().unwrap();

    if total == num_subs * min {
        // all balanced!
        return None;
    } else if total - num_subs * min > num_subs * max - total {
        // one is too small
        let it: &str = prog.subprogs.iter()
            .filter(|&p| progs[p].total_weight == min)
            .next().unwrap();
        return Some((it, max));
    } else {
        // one is too big
        let it: &str = prog.subprogs.iter()
            .filter(|&p| progs[p].total_weight == max)
            .next().unwrap();
        return Some((it, min));
    }
}

fn resize(progs: &HashMap<&str, Program>, name: &str) -> u32 {
    let mut curr = name;
    let mut new_size = None;

    loop {
        match check_subs(progs, curr) {
            None => {
                match new_size {
                    None => return 0,
                    Some(k) => {
                        let prog = &progs[curr];
                        let sub_weight = prog.total_weight - prog.weight;
                        let new_weight = k as i32 - sub_weight as i32;
                        return new_weight.abs() as u32;
                    },
                }
            },
            Some((sub, k)) => {
                new_size = Some(k);
                curr = sub;
            },
        }
    }
}

fn solve(input: &str) -> (String, u32) {
    let mut prog_list = parse_input(input);
    let mut rev_map: HashMap<&str, &str> = HashMap::new();
    for (name, ref prog) in prog_list.iter() {
        for subname in prog.subprogs.iter() {
            rev_map.insert(subname, name);
        }
    }
    let mut root: &str = rev_map.keys().next().unwrap();
    while let Some(x) = rev_map.get(root) {
        root = x;
    }

    find_total_weight(&mut prog_list, root);
    let part2 = resize(&prog_list, root);

    (String::from(root), part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INPUT: &'static str = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
";

    #[test]
    fn parsing() {
        let test_map: HashMap<&str, Program> = vec![
            ("pbga", Program{weight:66, total_weight:0, subprogs:vec![]}),
            ("fwft", Program{weight:72, total_weight:0, subprogs:vec!["ktlj", "cntj", "xhth"]}),
            ].into_iter().collect();
        assert_eq!(parse_input("pbga (66)\nfwft (72) -> ktlj, cntj, xhth\n"),
                   test_map);
    }

    #[test]
    fn example1() {
        let (part1, part2) = solve(EX_INPUT);
        assert_eq!("tknk", part1);
        assert_eq!(60, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day07.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day07.txt"),
                   format!("{:?}", x));
    }
}
