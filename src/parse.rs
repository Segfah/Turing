use std::collections::HashMap;
use serde::Deserialize;
use core::cmp::Ordering;

#[derive(Deserialize, Debug)]
pub struct TuringMachine {
    pub name: String,
    pub alphabet: Vec<char>,
    pub blank: char,
    pub states: Vec<String>,
    pub initial: String,
    pub finals: Vec<String>,
    pub transitions: HashMap<String, Vec<Transition>>
}

impl TuringMachine {
    /// Confirm the machine validity
    ///
    /// Verify initial states is describe in the states field
    /// Verify all finals states are describe in the states field
    /// Verify all transitions start from an already defined state
    /// Verify all transitions use a char from the alphabet as read
    /// Verify all transitions go to an already defined state
    ///
    /// NOTE: There may be a cleaner way to perform these check
    /// and get a more precise configuration error
    pub fn is_valid(&self) -> Result<(), ConfigErr> {
        // Check that initial state exist
        if !self.states.contains(&self.initial) {
            return Err(ConfigErr::InvalidConfig(
                format!("Initial state \'{}\' not found in \'states\' field",
                        self.initial)));
        }

        // Check that all finals states exist
        if !self.finals.iter().all(|item| self.states.contains(item)) {
            return Err(ConfigErr::InvalidConfig(
                    format!("Finals states aren't all describe in the \'state\' field\n{:?}\n{:?}",
                            self.states, self.finals)
                    ))
        }

        // Check that all transitions start from an existing state
        // and transit to an existing one
        let transition_checker = |item: (&String, &Vec<Transition>)| -> bool {
            self.states.contains(item.0) 
                && item.1.iter().all(|tran| self.validate_transition(tran))
        };
        if !self.transitions.iter().all(transition_checker) {
            return Err(ConfigErr::InvalidConfig(
                    format!("Invalid transition description.
to_state: this field should contains an existing state
read: this field should contain a char present in the alphabet
Please verify each transitions")));
        }
        
        Ok(())
    }

    /// Take a transition and confirm the presence of 'read' and 'to_state'
    /// inside the alphabet and the states respectively
    pub fn validate_transition(&self, transition: &Transition) -> bool {
        self.alphabet.contains(&transition.read)
            && self.states.contains(&transition.to_state)
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_transition = |elem: (&String, &Vec<Transition>)| {
                elem.1.iter().map(|tran| { format!("({}, {}) -> ({}, {}, {:?})",
                    elem.0, tran.read, tran.to_state, tran.write, tran.action)
                })
                .collect::<Vec<String>>().join("\n")
        };
        let format_transition = self.transitions
                                    .iter()
                                    .map(format_transition)
                                    .collect::<Vec<String>>()
                                    .join("\n");
        write!(f, "Alphabet: {:?}
States: {:?}
Initial: {}
Finals: {:?}
{}",
self.alphabet, self.states, self.initial, self.finals, format_transition)
    }
}

#[derive(Deserialize, Debug)]
pub struct Transition {
    pub read: char,
    pub to_state: String,
    pub write: char,
    pub action: Action
}

#[derive(Deserialize, Debug)]
pub enum Action {
    RIGHT,
    LEFT
}


#[derive(Debug)]
pub enum ConfigErr {
    MissingArgument,
    TooMuchArgument,
    InvalidConfig(String),
}

use std::fmt;
impl fmt::Display for ConfigErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingArgument => write!(f, "Missing argument"),
            Self::TooMuchArgument => write!(f, "Received too much arguments"),
            Self::InvalidConfig(err) => write!(f, "{}", err)
        }
    }
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
                        filename, err)
                )),
        Ok(data) => {
            serde_json::from_str(data.as_str())
                .map_err(|err| {
                    ConfigErr::InvalidConfig(format!(
                        "Error while parsing file \'{}\':\n{}",
                        filename, err)
                    )
                })
        }
    }
}

/// Parse the machine configuration given in parameters
///
/// Load the configuration from file then validate the configuration
pub fn parse_machine() -> Result<TuringMachine, ConfigErr> {
    let machine = load_config_json()?;

    machine.is_valid()?;

    Ok(machine)
}
