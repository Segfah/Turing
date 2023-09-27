#![feature(exclusive_range_pattern)]

extern crate colored;
mod interpretor;
mod parse;
use clap::{Command, Arg};


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
    //println!("Input: {}", input);
    //let input = "111-11=".to_string();
    match config {
        Ok(machine) => {
            println!("{}\n{:=<80}", machine, "");
            let mut emulator = interpretor::TuringInterpret::new(machine, input.clone());
            println!("[{0:^padding$}]", input, padding=78);
            println!("{:=<80}", "");
            emulator.run();
        },
        Err(err) => eprintln!("{}", err)
    }
}
