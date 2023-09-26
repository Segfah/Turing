mod parse;

fn main() {
    let config = parse::parse_machine();
    match config {
        Ok(machine) => println!("{}", machine),
        Err(err) => eprintln!("{}", err)
    }
}
