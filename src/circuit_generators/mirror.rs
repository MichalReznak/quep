use std::fmt::Write;


use async_trait::async_trait;
use derive_more::Constructor;



use crate::traits::CircuitGenerator;
use crate::Error;

const CIRCUIT_TEMPLATE: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[%WIDTH%];
creg c[%WIDTH%];

%RESET%

barrier q;

%HALF_DEPTH_CIRCUIT%

barrier q;

%HALF_DEPTH_INVERSE_CIRCUIT%

barrier q;

measure q -> c;
"#;


#[derive(Constructor)]
pub struct MirrorCircuitGenerator;


// Randomized mirror benchmarking with some restrictions:
// Always the result should be all zeros
// Second part of algorithm is always inverse to the first part in everything
// Length is counted as 2d.

#[async_trait]
impl CircuitGenerator for MirrorCircuitGenerator {
    async fn generate(&self, i: i32, j: i32) -> Result<Option<String>, Error> {
        // if i > 4 || j > 4 {
        //     return Ok(None);
        // }

        let pauli_gates = [
            "id",
            "x",
            "y",
            "z",
        ];

        let clifford_gates = [
            "id",
            "x",
            "y",
            "z",
            "h",
            "s",
        ];

        let clifford_gates_inv = [
            "id",
            "x",
            "y",
            "z",
            "h",
            "sdg",
        ];

        let clifford_gates_2 = [
            "cx",
            "cz",
            "swap",
        ];

        let clifford_gates_2_inv = [
            "cx",
            "cz",
            "swap",
        ];

        let i = i + 1;
        let j = j + 2;
        let circuit = CIRCUIT_TEMPLATE.replace("%WIDTH%", &i.to_string());


        let mut resets = String::new();
        for i in 0..i {
            write!(&mut resets, "reset q[{}];\n", i).unwrap();
        }
        let circuit = circuit.replace("%RESET%", &resets);

        let mut c_gates = vec![];
        let mut p_gates = vec![];

        let mut a = 0;
        let mut half_cir = String::new();
        for _ in 0..j {
            for i in 0..i {
                let c_gate_index = a as usize % clifford_gates.len();
                write!(&mut half_cir, "{} q[{}];\n", clifford_gates[c_gate_index], i).unwrap();
                c_gates.push(c_gate_index);

                let p_gate_index = (a + 1) as usize % pauli_gates.len();
                write!(&mut half_cir, "{} q[{}];\n", pauli_gates[p_gate_index], i).unwrap();
                p_gates.push(p_gate_index);
                a += 1;
            }
            write!(&mut half_cir, "\n").unwrap();
        }
        let circuit = circuit.replace("%HALF_DEPTH_CIRCUIT%", &half_cir);

        let mut half_cir_inv = String::new();
        for _ in 0..j {
            for i in (0..i).rev() {
                let c_gate_index = c_gates.pop().unwrap();
                let p_gate_index = p_gates.pop().unwrap();
                write!(&mut half_cir_inv, "{} q[{}];\n", pauli_gates[p_gate_index], i).unwrap();
                write!(&mut half_cir_inv, "{} q[{}];\n", clifford_gates_inv[c_gate_index], i).unwrap();
                a -= 1;
            }
            write!(&mut half_cir_inv, "\n").unwrap();
        }
        let circuit = circuit.replace("%HALF_DEPTH_INVERSE_CIRCUIT%", &half_cir_inv);

        println!("{circuit}");

        Ok(Some(circuit))
    }
}
