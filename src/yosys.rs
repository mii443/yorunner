use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Port {
    pub direction: String,
    pub bits: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cell {
    pub hide_name: usize,
    #[serde(rename = "type")]
    pub type_: String,
    pub parameters: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
    pub port_directions: HashMap<String, String>,
    pub connections: HashMap<String, Vec<usize>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Netname {
    pub hide_name: usize,
    pub bits: Vec<usize>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Module {
    pub attributes: HashMap<String, String>,
    pub ports: HashMap<String, Port>,
    pub cells: HashMap<String, Cell>,
    pub netnames: HashMap<String, Netname>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Circuit {
    pub creator: String,
    pub modules: HashMap<String, Module>,
}