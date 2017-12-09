use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Instr<'a> {
    reg: &'a str,
    arg: i32,
    creg: &'a str,
    cop: CmpOp,
    carg: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum CmpOp { LT, LE, EQ, NE, GE, GT }
use self::CmpOp::*;

fn parse_input(s: &str) -> Vec<Instr> {
    s.lines().map(|line| {
            let mut s = line.split_whitespace();
            let reg = s.next().unwrap();
            let op = s.next().unwrap();
            let val: i32 = s.next().unwrap().parse().unwrap();
            let arg = match op {
                "inc" => val,
                "dec" => -val,
                _ => panic!("unknown operation: {}", op)
            };
            s.next(); // drop "if"
            let creg = s.next().unwrap();
            let cop = s.next().unwrap();
            let cop = match cop {
                "<"  => LT,
                "<=" => LE,
                "==" => EQ,
                "!=" => NE,
                ">=" => GE,
                ">"  => GT,
                _ => panic!("unknown comparison: {}", cop)
            };
            let carg = s.next().unwrap().parse().unwrap();
            Instr { reg, arg, creg, cop, carg }
        }).collect()
}

fn solve(input: &str) -> (i32, i32) {
    let instrs = parse_input(input);
    let mut regs = HashMap::new();
    let mut max = 0;

    for i in instrs {
        let creg: i32 = *regs.entry(i.creg).or_insert(0);
        let cond = match i.cop {
            LT => creg < i.carg,
            LE => creg <= i.carg,
            EQ => creg == i.carg,
            NE => creg != i.carg,
            GE => creg >= i.carg,
            GT => creg > i.carg,
        };
        if cond {
            let reg = regs.entry(i.reg).or_insert(0);
            *reg += i.arg;
            if *reg > max {
                max = *reg;
            }
        }
    }
    (*regs.values().max().unwrap(), max)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the largest value in a register is {}", part1);
    println!("the highest value ever held during processing is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
            vec![
                Instr{reg:"b",arg:5,creg:"a",cop:GT,carg:1},
                Instr{reg:"a",arg:1,creg:"b",cop:LT,carg:5},
                Instr{reg:"c",arg:10,creg:"a",cop:GE,carg:1},
                Instr{reg:"c",arg:-20,creg:"c",cop:EQ,carg:10},
            ]);
    }

    #[test]
    fn example1() {
        let (part1,part2) = solve(EXAMPLE);
        assert_eq!(1, part1);
        assert_eq!(10, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day08.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day08.txt"),
                   format!("{:?}", x));
    }
}
