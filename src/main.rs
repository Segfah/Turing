#![feature(exclusive_range_pattern)]

extern crate colored;
mod interpretor;
mod parse;

use clap::{Command, Arg};
use interpretor::TuringInterpret;

pub fn validate_args() {
    let _matches = Command::new("Turing")
        .arg(Arg::new("jsonfile")
            .required(true)
            .help("json description of the machine")
            .index(1))
        .arg(Arg::new("input")
            .required(true)
            .help("input of the machine")
            .index(2))
        .get_matches();
}

fn main() {
    validate_args();
    let config = parse::parse_machine();
    let input = parse::parse_input();

    match config {
        Ok(machine) => {
            println!("{}\n{:=<80}", machine, "");
            println!("[{0:^padding$.78}]", input, padding=78);
            println!("{:=<80}", "");

            let valid_input = |elem: char| -> bool {
                machine.alphabet.contains(&elem) };
            if !input.chars().all(valid_input) {
                eprintln!("Input contains illegal character");
                eprintln!("Alphabet {:?}", machine.alphabet);
                return ;
            }

            TuringInterpret::new(machine, input).run();
        },
        Err(err) => eprintln!("{}", err)
    }
}
