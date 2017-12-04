use std::collections::HashSet;
use std::hash::Hash;

fn parse_input(s: &str) -> Vec<Vec<&str>> {
    s.lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

fn all_unique<I: Iterator>(mut xs: I) -> bool where
    I::Item: Eq + Hash
{
    let mut set = HashSet::new();
    xs.all(|x| set.insert(x))
}

fn sort_word(w: &str) -> Vec<char> {
    let mut v: Vec<_> = w.chars().collect();
    v.sort();
    v
}

fn solve(input: &str) -> (usize, usize) {
    let ps = parse_input(input);
    let part1 = ps.iter()
        .filter(|p| all_unique(p.iter()))
        .count();
    let part2 = ps.iter()
        .filter(|p| all_unique(p.iter().map(|w| sort_word(w))))
        .count();
    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("Valid passphrases (weak): {}", part1);
    println!("Valid passphrases (strong): {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse_input("aa bb cc dd ee\n\
                                aa bb cc dd aa\n\
                                aa bb cc dd aaa\n\
                                "),
                   vec![vec!["aa","bb","cc","dd","ee"],
                        vec!["aa","bb","cc","dd","aa"],
                        vec!["aa","bb","cc","dd","aaa"]]);
    }

    #[test]
    fn example1() {
        assert_eq!(2, solve("aa bb cc dd ee\n\
                             aa bb cc dd aa\n\
                             aa bb cc dd aaa\n\
                             ").0);
    }

    #[test]
    fn example2() {
        assert_eq!(3, solve("abcde fghij\n\
                             abcde xyz ecdab\n\
                             a ab abc abd abf abj\n\
                             iiii oiii ooii oooi oooo\n\
                             oiii ioii iioi iiio\n\
                             ").1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day04.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day04.txt"),
                   format!("{:?}", x));
    }
}
