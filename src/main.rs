mod parse;

use parse::TuringMachine;
use core::cmp::Ordering;

#[derive(Debug)]
pub enum ConfigErr {
    MissingArgument,
    TooMuchArgument,
    InvalidConfig(String),
}

/// Cargador de configuración
///
/// Cargue el archivo de configuración y devuelva un resultado
/// que contenga el valor o el error de análisis
pub fn load_config_json() -> Result<TuringMachine, ConfigErr> {
    let env: Vec<String> = std::env::args().collect();
    match env.len().cmp(&2) {
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
