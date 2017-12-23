extern crate advent2017;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use advent2017::day01;
use advent2017::day02;
use advent2017::day03;
use advent2017::day04;
use advent2017::day05;
use advent2017::day06;
use advent2017::day07;
use advent2017::day08;
use advent2017::day09;
use advent2017::day10;
use advent2017::day11;
use advent2017::day12;
use advent2017::day13;
use advent2017::day14;
use advent2017::day15;
use advent2017::day16;
use advent2017::day17;
use advent2017::day18;
use advent2017::day19;
use advent2017::day20;
use advent2017::day21;
use advent2017::day22;

struct Config {
    target: String,
    input_file: String,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // discard exe name
        let target = match args.next() {
            Some(arg) => arg,
            None => return Err("requires DAY argument"),
        };
        let input_file = match args.next() {
            Some(arg) => arg,
            None => format!("inputs/{}.txt", target),
        };
        Ok(Config { target, input_file })
    }
}

fn get_input(input_file: String) -> Result<String, std::io::Error> {
    let mut f = File::open(input_file)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let input = get_input(cfg.input_file).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    match &*cfg.target {
        "day01" => day01::run(&input),
        "day02" => day02::run(&input),
        "day03" => day03::run(&input),
        "day04" => day04::run(&input),
        "day05" => day05::run(&input),
        "day06" => day06::run(&input),
        "day07" => day07::run(&input),
        "day08" => day08::run(&input),
        "day09" => day09::run(&input),
        "day10" => day10::run(&input),
        "day11" => day11::run(&input),
        "day12" => day12::run(&input),
        "day13" => day13::run(&input),
        "day14" => day14::run(&input),
        "day15" => day15::run(&input),
        "day16" => day16::run(&input),
        "day17" => day17::run(&input),
        "day18" => day18::run(&input),
        "day19" => day19::run(&input),
        "day20" => day20::run(&input),
        "day21" => day21::run(&input),
        "day22" => day22::run(&input),
        _ => {
            eprintln!("unknown day");
            process::exit(1);
        }
    }
}
