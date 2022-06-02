use std::fmt::Write;

use async_trait::async_trait;
use log::debug;

use crate::args::CliArgsCircuit;
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

pub struct MirrorCircuitGenerator {
    args: CliArgsCircuit,
}

// Structured mirror benchmarking with some restrictions:
// Always the result should be all zeros
// Second part of algorithm is always inverse to the first part in everything
// Length is counted as 2d.

impl MirrorCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for MirrorCircuitGenerator {
    async fn generate(
        &mut self,
        i: i32,
        j: i32,
        iter: i32,
        _: bool,
    ) -> Result<Option<String>, Error> {
        let iter = if self.args.rand { iter } else { 0 };

        let pauli_gates = ["id", "x", "y", "z"];

        let clifford_gates = ["h", "s", "id", "x", "y", "z"];

        let clifford_gates_inv = ["h", "sdg", "id", "x", "y", "z"];

        let clifford_gates_2 = ["cx", "cz", "swap"];

        let i = i + 1;
        let j = j + 1;
        let circuit = CIRCUIT_TEMPLATE.replace("%WIDTH%", &i.to_string());

        let mut resets = String::new();
        for i in 0..i {
            writeln!(&mut resets, "reset q[{}];", i)?;
        }
        let circuit = circuit.replace("%RESET%", &resets);

        let mut inv_gates = vec![];

        let c_len = clifford_gates.len();
        let c_len2 = c_len + clifford_gates_2.len();

        let mut a = iter;
        let mut b = iter;
        let mut half_cir = String::new();
        let mut skip = false;
        for _ in 0..j {
            for ii in 0..i {
                let p_gate_index = b as usize % pauli_gates.len();
                let c_gate_index = a as usize % c_len2;
                b += 1;

                if skip {
                    skip = false;
                }
                else {
                    let mut s = String::new();
                    if c_gate_index < c_len {
                        writeln!(&mut half_cir, "{} q[{}];", clifford_gates[c_gate_index], ii)?;
                        writeln!(&mut s, "{} q[{}];", clifford_gates_inv[c_gate_index], ii)?;
                        a += 1;
                    }
                    // NO space for double gate
                    else if ii == i - 1 {
                        writeln!(
                            &mut half_cir,
                            "{} q[{}];",
                            clifford_gates[c_gate_index - c_len],
                            ii
                        )?;
                        writeln!(
                            &mut s,
                            "{} q[{}];",
                            clifford_gates_inv[c_gate_index - c_len],
                            ii
                        )?;
                    }
                    else {
                        writeln!(
                            &mut half_cir,
                            "{} q[{}], q[{}];",
                            clifford_gates_2[c_gate_index - c_len],
                            ii,
                            ii + 1
                        )?;
                        writeln!(
                            &mut s,
                            "{} q[{}], q[{}];",
                            clifford_gates_2[c_gate_index - c_len],
                            ii,
                            ii + 1
                        )?;
                        a += 1;
                        skip = true;
                    }
                    inv_gates.push(s);
                }

                let mut s = String::new();
                writeln!(&mut half_cir, "{} q[{}];", pauli_gates[p_gate_index], ii)?;
                writeln!(&mut s, "{} q[{}];", pauli_gates[p_gate_index], ii)?;
                inv_gates.push(s);
            }
            writeln!(&mut half_cir)?;
        }
        let circuit = circuit.replace("%HALF_DEPTH_CIRCUIT%", &half_cir);

        let mut half_cir_inv = String::new();
        inv_gates.reverse();
        for ss in inv_gates {
            write!(&mut half_cir_inv, "{}", ss)?;
        }

        let circuit = circuit.replace("%HALF_DEPTH_INVERSE_CIRCUIT%", &half_cir_inv);

        debug!("{circuit}");

        Ok(Some(circuit))
    }
}
