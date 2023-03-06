use std::{fs::File, io::Read};

use crate::yosys::Circuit;

mod yosys;

fn main() {
    let mut file = File::open("../tfhe-verilog/tfhe_test.json").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let circuit: Circuit = serde_json::from_str(&contents).unwrap();

    println!("{:?}", circuit);
}
