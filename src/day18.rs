use std::collections::VecDeque;

type RegID = usize;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Val {
    Reg(RegID),
    Imm(i64),
}
use self::Val::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instr {
    Snd(Val),
    Set(RegID, Val),
    Add(RegID, Val),
    Mul(RegID, Val),
    Mod(RegID, Val),
    Rcv(RegID),
    Jgz(Val, Val),
}
use self::Instr::*;

fn parse_val(s: &str) -> Val {
    if let Ok(n) = s.parse() {
        Imm(n)
    } else {
        Reg(parse_reg(s))
    }
}

fn parse_reg(s: &str) -> RegID {
    let bytes = s.as_bytes();
    if bytes.len() != 1 || bytes[0] < b'a' || bytes[0] > b'z' {
        panic!("invalid register: {}", s);
    }
    (bytes[0] - b'a') as RegID
}

fn parse_input(s: &str) -> Vec<Instr> {
    s.lines().map(|line| {
            let mut x = line.split_whitespace();
            let op = x.next().unwrap();
            match op {
                "snd" => Snd(parse_val(x.next().unwrap())),
                "set" => Set(parse_reg(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                "add" => Add(parse_reg(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                "mul" => Mul(parse_reg(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                "mod" => Mod(parse_reg(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                "rcv" => Rcv(parse_reg(x.next().unwrap())),
                "jgz" => Jgz(parse_val(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                _ => panic!("unknown instruction: '{}'", line)
            }
        }).collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ProgramStatus {
    Halted,
    Running,
    Blocked(RegID),
}
use self::ProgramStatus::*;

#[derive(Debug)]
struct Program {
    status: ProgramStatus,
    pc: i64,
    regs: [i64; 26],
    instrs: Vec<Instr>,
    msgs: VecDeque<i64>,
}

impl Program {
    fn new(id: i64, instrs: &[Instr]) -> Program {
        let mut p = Program {
            status: Running,
            regs: [0; 26],
            pc: 0,
            instrs: instrs.to_vec(),
            msgs: VecDeque::new(),
        };
        const REG_P: RegID = (b'p' - b'a') as RegID;
        p.regs[REG_P] = id;
        p
    }

    fn get_val(&self, v: Val) -> i64 {
        match v {
            Reg(r) => self.regs[r],
            Imm(n) => n,
        }
    }

    fn execute(&mut self, instr: Instr) -> Option<i64> {
        let mut result = None;
        match instr {
            Snd(x) => result = Some(self.get_val(x)),
            Set(x,y) => self.regs[x] = self.get_val(y),
            Add(x,y) => self.regs[x] += self.get_val(y),
            Mul(x,y) => self.regs[x] *= self.get_val(y),
            Mod(x,y) => self.regs[x] %= self.get_val(y),
            Rcv(x) =>
                if let Some(y) = self.msgs.pop_front() {
                    self.regs[x] = y;
                } else {
                    self.status = Blocked(x);
                }
            Jgz(x,y) =>
                if self.get_val(x) > 0 {
                    self.pc += self.get_val(y) - 1 ;
                }
        }
        self.pc += 1;
        result
    }

    fn step(&mut self) -> Option<i64> {
        let mut result = None;
        match self.status {
            Running => {
                if self.pc < 0 || (self.pc as usize) >= self.instrs.len() {
                    self.status = Halted;
                } else {
                    let i = self.instrs[self.pc as usize];
                    result = self.execute(i);
                }
            }
            _ => (),
        }
        result
    }
    
    fn take_msg(&mut self, y: i64) {
        if let Blocked(r) = self.status {
            self.regs[r] = y;
            self.status = Running;
        } else {
            self.msgs.push_back(y);
        }
    }
}

fn run_solo(song: &[Instr]) -> i64 {
    let mut freq = 0;
    let mut prog = Program::new(0, song);
    while prog.status == Running {
        if let Some(f) = prog.step() {
            freq = f;
        }
        // If it executed a rcv with a zero value, continue
        if let Blocked(r) = prog.status {
            if prog.regs[r] == 0 {
                prog.status = Running;
            }
        }
    }
    freq
}

fn run_duet(song: &[Instr]) -> u32 {
    let mut prog0 = Program::new(0, song);
    let mut prog1 = Program::new(1, song);
    let mut prog1_sends = 0;
    
    while prog0.status == Running || prog1.status == Running {
        if let Some(m) = prog0.step() {
            prog1.take_msg(m);
        }
        if let Some(m) = prog1.step() {
            prog0.take_msg(m);
            prog1_sends += 1;
        }
    }
    prog1_sends
}

fn solve(input: &str) -> (i64, u32) {
    let song = parse_input(input);
    let part1 = run_solo(&song);
    let part2 = run_duet(&song);
    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";
    
    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
                   vec![Set(0, Imm(1)),
                        Add(0, Imm(2)),
                        Mul(0, Reg(0)),
                        Mod(0, Imm(5)),
                        Snd(Reg(0)),
                        Set(0, Imm(0)),
                        Rcv(0),
                        Jgz(Reg(0), Imm(-1)),
                        Set(0, Imm(1)),
                        Jgz(Reg(0), Imm(-2))]);
    }

    #[test]
    fn example1() {
        let part1 = run_solo(parse_input(EXAMPLE));
        assert_eq!(4, part1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day18.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day18.txt"),
                   format!("{:?}", x));
    }
}
