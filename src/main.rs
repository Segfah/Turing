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
    match config {
        Ok(machine) => println!("{}", machine),
        Err(err) => eprintln!("{}", err)
    }
}
