type RegID = usize;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Val {
    Reg(RegID),
    Imm(i64),
}
use self::Val::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instr {
    Set(RegID, Val),
    Sub(RegID, Val),
    Mul(RegID, Val),
    Jnz(Val, Val),
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
    if bytes.len() != 1 || bytes[0] < b'a' || bytes[0] > b'h' {
        panic!("invalid register: {}", s);
    }
    (bytes[0] - b'a') as RegID
}

fn parse_input(s: &str) -> Vec<Instr> {
    s.lines().map(|line| {
            let mut x = line.split_whitespace();
            let op = x.next().unwrap();
            match op {
                "set" => Set(parse_reg(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                "sub" => Sub(parse_reg(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                "mul" => Mul(parse_reg(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                "jnz" => Jnz(parse_val(x.next().unwrap()),
                             parse_val(x.next().unwrap())),
                _ => panic!("unknown instruction: '{}'", line)
            }
        }).collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ProgramStatus {
    Halted,
    Running,
}
use self::ProgramStatus::*;

const NUM_REGS: usize = 8;

#[derive(Debug)]
struct Program {
    status: ProgramStatus,
    pc: i64,
    regs: [i64; NUM_REGS],
    instrs: Vec<Instr>,
    num_muls: u32,
}

impl Program {
    fn new(instrs: &[Instr]) -> Program {
        Program {
            status: Running,
            regs: [0; NUM_REGS],
            pc: 0,
            instrs: instrs.to_vec(),
            num_muls: 0,
        }
    }

    fn get_val(&self, v: Val) -> i64 {
        match v {
            Reg(r) => self.regs[r],
            Imm(n) => n,
        }
    }

    fn execute(&mut self, instr: Instr) {
        match instr {
            Set(x,y) => self.regs[x] = self.get_val(y),
            Sub(x,y) => self.regs[x] -= self.get_val(y),
            Mul(x,y) => {
                self.regs[x] *= self.get_val(y);
                self.num_muls += 1;
            }
            Jnz(x,y) =>
                if self.get_val(x) != 0 {
                    self.pc += self.get_val(y) - 1 ;
                }
        }
        self.pc += 1;
    }

    fn step(&mut self) {
        match self.status {
            Running => {
                if self.pc < 0 || (self.pc as usize) >= self.instrs.len() {
                    self.status = Halted;
                } else {
                    let i = self.instrs[self.pc as usize];
                    self.execute(i);
                }
            }
            _ => (),
        }
    }
}

fn run_part1(instrs: &[Instr]) -> u32 {
    let mut prog = Program::new(instrs);
    while prog.status == Running {
        prog.step();
    }
    prog.num_muls
}

fn sieve(limit: usize) -> Vec<bool> {
    let mut s = vec![true; limit];
    s[0] = false;
    s[1] = false;
    for k in 2..limit/2 { s[2*k] = false; }
    let mut c = 3;
    while c*c < limit {
        if s[c] {
            let mut d = c*c;
            while d < limit {
                s[d] = false;
                d += c;
            }
        }
        c += 2;
    }
    s
}

fn do_part2() -> u32 {
    const B: usize = 79*100 + 100_000;
    const C: usize = B + 17_000;

    let primes = sieve(C+1);
    let mut x = B;
    let mut composites = 0;
    while x <= C {
        if !primes[x] {
            composites += 1;
        }
        x += 17;
    }
    composites
}

fn solve(input: &str) -> (u32, u32) {
    let instrs = parse_input(input);
    let part1 = run_part1(&instrs);
    let part2 = do_part2();
    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the number of mul instructions executed is {}", part1);
    println!("the final value of register h is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day23.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day23.txt"),
                   format!("{:?}", x));
    }
}
