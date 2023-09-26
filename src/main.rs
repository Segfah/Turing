mod parse;

use parse::TuringMachine;
use core::cmp::Ordering;
use clap::{Command, Arg};

#[derive(Debug)]
pub enum ConfigErr {
    MissingArgument,
    TooMuchArgument,
    InvalidConfig(String),
}

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

/// Cargador de configuración
///
/// Cargue el archivo de configuración y devuelva un resultado
/// que contenga el valor o el error de análisis
pub fn load_config_json() -> Result<TuringMachine, ConfigErr> {
    validate_args();
    let env: Vec<String> = std::env::args().collect();
    match env.len().cmp(&3) {
        Ordering::Greater => return Err(ConfigErr::TooMuchArgument),
        Ordering::Less => return Err(ConfigErr::MissingArgument),
        _ => {}
    }

    let filename: &String = &env[1];
    let contents = std::fs::read_to_string(filename);
    match contents {
        Err(err) => Err(ConfigErr::InvalidConfig(format!(
            "Error while parsing file \'{}\': {}",
            filename, err
        ))),
        Ok(data) => {
            let machine = serde_json::from_str(data.as_str());
            machine.map_err(|err| {
                ConfigErr::InvalidConfig(format!(
                    "Error while parsing file \'{}\':\n{}",
                    filename, err
                ))
            })
        }
    }
}

fn main() {
    let config = load_config_json();
    match config {
        Ok(machine) => println!("{:?}", machine),
        Err(err) => eprintln!("{:?}", err)
    }
}
