use std::fmt::Write;

use async_trait::async_trait;

use log::debug;
use rand::distributions::{Distribution, Uniform};

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

pub struct RandMirrorCircuitGenerator {
    args: CliArgsCircuit,
}

impl RandMirrorCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

// Randomized mirror benchmarking with some restrictions:
// Always the result should be all zeros
// Second part of algorithm is always inverse to the first part in everything
// Length is counted as 2d.
// It is using **uniform sampling**

// TODO missing rand

#[async_trait]
impl CircuitGenerator for RandMirrorCircuitGenerator {
    async fn generate(
        &mut self,
        i: i32,
        j: i32,
        _iter: i32,
        _: bool,
    ) -> Result<Option<String>, Error> {
        let pauli_gates = ["id", "x", "y", "z"];

        let clifford_gates = ["h", "s", "id", "x", "y", "z"];

        let clifford_gates_inv = ["h", "sdg", "id", "x", "y", "z"];

        let clifford_gates_2 = ["cx", "cz", "swap"];

        let mut rng = rand::thread_rng();
        let p_rand: Uniform<usize> = Uniform::from(0..4);
        let c_rand: Uniform<usize> = Uniform::from(0..9);

        let i = i + 1;
        let j = j + 1;
        let circuit = CIRCUIT_TEMPLATE.replace("%WIDTH%", &i.to_string());

        let mut resets = String::new();
        for i in 0..i {
            write!(&mut resets, "reset q[{}];\n", i)?;
        }
        let circuit = circuit.replace("%RESET%", &resets);

        let mut inv_gates = vec![];

        let c_len = clifford_gates.len();

        let mut half_cir = String::new();
        let mut skip = false;
        for _ in 0..j {
            for ii in 0..i {
                let p_gate_index = p_rand.sample(&mut rng);
                let c_gate_index = c_rand.sample(&mut rng);

                if skip {
                    skip = false;
                }
                else {
                    let mut s = String::new();
                    if c_gate_index < c_len {
                        write!(&mut half_cir, "{} q[{}];\n", clifford_gates[c_gate_index], ii)?;
                        write!(&mut s, "{} q[{}];\n", clifford_gates_inv[c_gate_index], ii)?;
                    }
                    // NO space for double gate
                    else if ii == i - 1 {
                        write!(
                            &mut half_cir,
                            "{} q[{}];\n",
                            clifford_gates[c_gate_index - c_len],
                            ii
                        )?;
                        write!(
                            &mut s,
                            "{} q[{}];\n",
                            clifford_gates_inv[c_gate_index - c_len],
                            ii
                        )?;
                    }
                    else {
                        write!(
                            &mut half_cir,
                            "{} q[{}], q[{}];\n",
                            clifford_gates_2[c_gate_index - c_len],
                            ii,
                            ii + 1
                        )?;
                        write!(
                            &mut s,
                            "{} q[{}], q[{}];\n",
                            clifford_gates_2[c_gate_index - c_len],
                            ii,
                            ii + 1
                        )?;
                        skip = true;
                    }
                    inv_gates.push(s);
                }

                let mut s = String::new();
                write!(&mut half_cir, "{} q[{}];\n", pauli_gates[p_gate_index], ii)?;
                write!(&mut s, "{} q[{}];\n", pauli_gates[p_gate_index], ii)?;
                inv_gates.push(s);
            }
            write!(&mut half_cir, "\n")?;
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
