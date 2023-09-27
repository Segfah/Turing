
use crate::colored::Colorize;

pub struct TuringInterpret {
    pub config: crate::parse::TuringMachine,
    pub state:  String,
    pub band:   Vec<char>,
    pub offset: usize
}

impl TuringInterpret {
    /// Create a machine interpretor given the input and it's configuration
    pub fn new(machine: crate::parse::TuringMachine, input: String) -> Self {
        let initial = machine.initial.clone();
        Self {
            config: machine,
            state: initial,
            band: input.chars().collect(),
            offset: 0
        }
    }

    /// Execute the next state transition, returning the old state or None
    pub fn step(&mut self) -> Option<String> {
        // Look for existing transition
        let transition = self.config.transitions
            .get(&self.state)?
            .iter()
            .find(|elem| elem.read == self.band[self.offset])?;

        self.print_current_step(&transition);

        self.band[self.offset] = transition.write;
        let old_state = self.state.clone();
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
                self.offset += 1;
                if self.offset == self.band.len() {
                    self.band.push(self.config.blank);
                }
            },
        };
        Some(old_state)
    }

    /// Execute steps until on final step is reached
    /// If the machine doesn't provide a final step or never reach it
    /// this can lead to an infinite loop and make the program panic due
    /// to stack overflow
    pub fn run(&mut self) {
        let old_state = self.step();
        match (old_state, self.is_in_final()) {
            (Some(_), false) => { 
                self.run();
            },
            (Some(_), true) => {
                println!("{:-^80}", " Finished ");
                self.print_band(80);
            },
            (None, _) => {
                println!("{:-^80}", " Finished ");
                self.print_band(80);
            },
        }
    }

    /// Print the whole step given the transition to perform.
    pub fn print_current_step(&self, transition: &crate::parse::Transition) {
        self.print_band(13);
        println!(
            "({}, {}) -> ({}, {}, {:?})",
            self.state,
            transition.read,
            transition.to_state,
            transition.write,
            transition.action
        );
    }

    /// Print the current band state, highlighting current offset
    pub fn print_band(&self, len: usize) {
        // Remove 3 to len for the open and closing braces
        // and the offset
        let len = len - 3;
        let lpadding = len / 2;
        let rpadding = len - lpadding;
        let left = {
            if self.offset < lpadding { &self.band[0..self.offset] }
            else { &self.band[self.offset - lpadding..self.offset] }
        };
        let right = {
            if self.band.len() <= self.offset + rpadding { &self.band[self.offset+1..] }
            else { &self.band[self.offset+1..self.offset + 1 + rpadding]}
        };
        print!("[");
        print!("{}", self.config.blank.to_string().repeat(lpadding - left.len()));
        left.iter().enumerate().for_each(|elem| print!("{}", elem.1));
        print!("{}", format!("{}", self.band[self.offset]).red());
        right.iter().enumerate().for_each(|elem| print!("{}", elem.1));
        print!("{}", self.config.blank.to_string().repeat(rpadding - right.len()));
        print!("] ");
    }

    pub fn is_in_final(&self) -> bool {
        self.config.finals.contains(&self.state)
    }
}
