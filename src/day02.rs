fn parse_input(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve(input: &str) -> (u32,u32) {
    let sheet = parse_input(input);
    let part1 = sheet.iter()
        .map(|row| (row.iter().max().unwrap() - row.iter().min().unwrap()))
        .sum();
    let part2 = sheet.iter()
        .map(|row| {
            for i in 0..row.len()-1 {
                for j in i+1..row.len() {
                    let u = row[i];
                    let v = row[j];
                    if u % v == 0 {
                        return u/v;
                    } else if v % u == 0 {
                        return v/u;
                    }
                }
            }
            0
        })
        .sum();
    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the checksum is {:?}", part1);
    println!("the sum of each row's result is {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse_input("0 1 10\n100 101 102\n"),
                   vec![vec![0, 1, 10], vec![100, 101, 102]]);
    }

    #[test]
    fn example1() {
        assert_eq!(18, solve("5 1 9 5\n7 5 3\n2 4 6 8\n").0);
    }

    #[test]
    fn example2() {
        assert_eq!(9, solve("5 9 2 8\n9 4 7 3\n3 8 6 5\n").1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day02.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day02.txt"),
                   format!("{:?}", x));
    }
}
