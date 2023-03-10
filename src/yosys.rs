use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not},
};

#[derive(Debug, Clone)]
pub struct Runner<T>
where
    T: BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Not<Output = T>
        + Display
        + Clone,
{
    pub module: Module,
    pub bits: Vec<T>,
}

impl<T> Runner<T>
where
    T: BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Not<Output = T>
        + Display
        + Clone,
{
    pub fn new(module: Module) -> Self {
        Self {
            module,
            bits: vec![],
        }
    }

    pub fn init(&mut self, input: Vec<T>) {
        let ports = &self.module.ports;
        let mut max_port = 0;
        for bits in ports {
            for bit in &bits.1.bits {
                if max_port < *bit {
                    max_port = *bit;
                }
            }
        }

        let nets = &self.module.netnames;
        for net in nets {
            for bit in &net.1.bits {
                if max_port < *bit {
                    max_port = *bit;
                }
            }
        }

        let f = input[0].clone() ^ input[0].clone();

        let mut input = input;
        input.reserve(max_port + 1 - input.len());

        for _ in input.len()..max_port + 1 {
            input.push(f.clone());
        }

        self.bits = input;
    }

    pub fn run(&mut self) {
        let net_id_regex = Regex::new(r"[0-9]+$").unwrap();
        let mut cells: Vec<(usize, Cell)> = Vec::with_capacity(self.module.cells.len());
        for (netname, net) in &self.module.netnames {
            if net.hide_name == 0 {
                continue;
            }

            let (cell, name) = self.get_cell(netname.clone());

            let id = net_id_regex.find(&name).unwrap().as_str();
            let id = usize::from_str_radix(id, 10).unwrap();

            cells.push((id, cell));
        }

        cells.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (_, cell) in cells {
            let input_connections = {
                let input_ports = cell.port_directions.iter().filter(|f| f.1 == "input");
                let mut connections = vec![];
                for port in input_ports {
                    let connection = cell.connections.iter().find(|f| f.0 == port.0).unwrap();
                    connections.push(connection);
                }
                connections
            };

            let output_connection = {
                let output_ports = cell.port_directions.iter().filter(|f| f.1 == "output");
                let (port, _): (&String, &String) = output_ports.last().unwrap();
                cell.connections.iter().find(|f| f.0 == port).unwrap()
            };

            match &*cell.type_ {
                "$and" => {
                    for (i, out) in output_connection.1.iter().enumerate() {
                        let a = &self.bits[input_connections[0].1[i]];
                        let b = &self.bits[input_connections[1].1[i]];
                        self.bits[*out] = a.clone() & b.clone();
                    }
                }
                "$or" => {
                    for (i, out) in output_connection.1.iter().enumerate() {
                        let a = &self.bits[input_connections[0].1[i]];
                        let b = &self.bits[input_connections[1].1[i]];
                        self.bits[*out] = a.clone() | b.clone();
                    }
                }
                "$xor" => {
                    for (i, out) in output_connection.1.iter().enumerate() {
                        let a = &self.bits[input_connections[0].1[i]];
                        let b = &self.bits[input_connections[1].1[i]];
                        self.bits[*out] = a.clone() ^ b.clone();
                    }
                }
                "$not" => {
                    for (i, out) in output_connection.1.iter().enumerate() {
                        let a = &self.bits[input_connections[0].1[i]];
                        self.bits[*out] = !a.clone();
                    }
                }
                _ => {
                    println!("not implemented");
                }
            }
        }
    }

    fn get_cell(&self, name: String) -> (Cell, String) {
        for (cell_name, cell) in &self.module.cells {
            if name.starts_with(cell_name) {
                return (cell.clone(), cell_name.clone());
            }
        }

        panic!("Cannot find cell.");
    }

    pub fn print_bits(&self) -> Vec<(String, Vec<T>)> {
        let mut res = vec![];
        for (netname, net) in &self.module.netnames {
            if net.hide_name == 1 {
                continue;
            }

            let mut buf = String::default();

            buf += &format!("{}: [", netname);

            let mut bits = vec![];
            for i in &net.bits {
                bits.push(self.bits[*i].clone());
                buf += &format!("{}, ", self.bits[*i]);
            }

            buf.pop();
            buf.pop();

            println!("{}]", buf);

            res.push((netname.clone(), bits));
        }

        res
    }
}

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
    pub parameters: HashMap<String, usize>,
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
    pub attributes: HashMap<String, serde_json::Value>,
    pub ports: HashMap<String, Port>,
    pub cells: HashMap<String, Cell>,
    pub netnames: HashMap<String, Netname>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Circuit {
    pub creator: String,
    pub modules: HashMap<String, Module>,
}
