use std::collections::HashMap;
use serde::Deserialize;

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

