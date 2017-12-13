use std::collections::HashMap;

fn parse_input(s: &str) -> HashMap<u32, Vec<u32>> {
    s.lines().map(|line| {
            let mut s = line.split(" <-> ");
            let id = s.next().unwrap().parse().unwrap();
            let links = s.next().unwrap();
            let links = links.split(", ").map(|m| m.parse().unwrap()).collect();
            (id, links)
        }).collect()
}

fn solve(input: &str) -> (u32, u32) {
    let network = parse_input(input);

    // Find connected components using DFS
    // Keep track of which group each program is in
    // and the size of each group.
    let mut visited: HashMap<u32,u32> = HashMap::new();
    let mut group_sizes: Vec<u32> = Vec::new();
    let mut to_visit: Vec<u32> = Vec::new();

    let mut group_id = 0;
    for &i in network.keys() {
        if visited.contains_key(&i) { continue; }
        to_visit.push(i);
        let mut group_size = 0;
        while let Some(prog_id) = to_visit.pop() {
            if visited.contains_key(&prog_id) { continue; }
            visited.insert(prog_id, group_id);
            group_size += 1;
            for &link in network[&prog_id].iter() {
                to_visit.push(link);
            }
        }
        group_id += 1;
        group_sizes.push(group_size);
    }

    let group_0_size = group_sizes[visited[&0] as usize];
    (group_0_size, group_id)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the size of program 0's group is {}", part1);
    println!("the total number of groups is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let t: HashMap<_,_> = vec![(0, vec![1]), (1, vec![0,1])].into_iter().collect();
        assert_eq!(parse_input("0 <-> 1\n1 <-> 0, 1\n"), t);
    }

    #[test]
    fn example1() {
        let (part1,part2) = solve("\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
");
        assert_eq!(6, part1);
        assert_eq!(2, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day12.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day12.txt"),
                   format!("{:?}", x));
    }
}
