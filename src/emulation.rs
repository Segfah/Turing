
use crate::colored::Colorize;

pub struct Emulator {
    pub config: crate::parse::TuringMachine,
    pub state:  String,
    pub band:   Vec<char>,
    pub offset: u32
}

impl Emulator {
    pub fn new(machine: crate::parse::TuringMachine, input: String) -> Self {
        let initial = machine.initial.clone();
        Self {
            config: machine,
            state: initial,
            band: input.chars().collect(),
            offset: 0
        }
    }

    pub fn step(&mut self) -> Option<String> {
        let transition = self.config
            .transitions[&self.state]
            .iter()
            .find(|elem| elem.read == self.band[self.offset as usize])?;
        println!("({}, {}) -> ({}, {}, {:?})",
                    self.state, transition.read, transition.to_state, transition.write, transition.action);
        self.band[self.offset as usize] = transition.write;
        self.state = transition.to_state.clone();
        match transition.action {
            crate::parse::Action::LEFT => {
                if self.offset == 0 {
                    self.band.insert(0, self.config.blank);
                } else {
                    self.offset -= 1;
                }
            },
            crate::parse::Action::RIGHT => {
                if self.offset as usize == self.band.len() {
                    self.band.push(self.config.blank);
                } else {
                    self.offset += 1;
                }
            },
        };
        Some(self.state.clone())
    }

    pub fn run(&mut self) {
        self.print_band();
        let new_state = self.step();
        match (new_state, self.is_in_final()) {
            (Some(_), false) => { 
                self.run();
            },
            (Some(_), true) => {
                println!("Finished");
            },
            (None, _) => {
                todo!()
            },
        }
    }

    pub fn print_band(&self) {
        let mut slice = &self.band[0..];
        let slice = match self.offset {
            0..10 => &self.band[0..],
            _ if self.offset as usize + 4 >= self.band.len() => &self.band[self.offset as usize - 5 ..],
            _ => &self.band[self.offset as usize - 5 .. self.offset as usize + 4]
        };
        print!("[");
        slice
        .iter()
        .enumerate()
        .for_each(|elem| {
            if elem.0 == self.offset as usize {
                print!("{}", format!("{}", elem.1).red());
            } else {
                print!("{}", elem.1);
            }
            });
        print!("] ");
    }

    pub fn is_in_final(&self) -> bool {
        self.config.finals.contains(&self.state)
    }
}
