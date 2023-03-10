use std::{fs::File, io::Read};

use yorunner::yosys::{Circuit, Runner};

fn main() {
    let mut file = File::open("../tfhe-verilog/tfhe_test.json").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let circuit: Circuit = serde_json::from_str(&contents).unwrap();

    let mut runner = Runner::new(circuit.modules.get("TFHE_TEST").unwrap().clone());
    runner.init(vec![
        false, false, //padding
        true, true, false, true, true, false, false, true, //A
        false, false, true, true, false, true, false, false, //B
    ]);

    runner.run();

    let _ = runner.print_bits();
}
