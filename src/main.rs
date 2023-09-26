#![feature(exclusive_range_pattern)]

extern crate colored;

mod parse;
mod emulation;

fn main() {
    let config = parse::parse_machine();
    let input = "111-11=".to_string();
    match config {
        Ok(machine) => {
            println!("{}\n{:=<80}", machine, "");
            let mut emulator = emulation::Emulator::new(machine, input);
            emulator.run();
        },
        Err(err) => eprintln!("{}", err)
    }
}
