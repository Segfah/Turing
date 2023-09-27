#![feature(exclusive_range_pattern)]

extern crate colored;

mod parse;
mod interpretor;

fn main() {
    let config = parse::parse_machine();
    let input = ".".to_string();
    let input = "11111111-1=".to_string();
    match config {
        Ok(machine) => {
            println!("{}\n{:=<80}", machine, "");
            let mut emulator = interpretor::TuringInterpret::new(machine, input);
            emulator.run();
            //(0..20).for_each(|elem| {
            //    emulator.step();
            //})
        },
        Err(err) => eprintln!("{}", err)
    }
}
