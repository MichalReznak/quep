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

%PREPARE%

barrier q;

%HALF_DEPTH_CIRCUIT%

barrier q;

%HALF_DEPTH_INVERSE_CIRCUIT%

barrier q;

%INVERT_PREPARE%

barrier q;

measure q -> c;
"#;


#[derive(Constructor)]
pub struct MirrorCircuitGenerator;

#[async_trait]
impl CircuitGenerator for MirrorCircuitGenerator {
    async fn generate(&self, i: i32, j: i32) -> Result<Option<String>, Error> {
        // if i > 4 || j > 4 {
        //     return Ok(None);
        // }

        let gates = [
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
            "h",
        ];

        let c_gates = [
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
            "s",
        ];

        let c_gates_inv = [
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
            "sdg",
        ];

        let i = i + 1;
        let j = j + 1;
        let circuit = CIRCUIT_TEMPLATE.replace("%WIDTH%", &i.to_string());


        let mut resets = String::new();
        for i in 0..i {
            write!(&mut resets, "reset q[{}];\n", i).unwrap();
        }
        let circuit = circuit.replace("%RESET%", &resets);


        let mut prepare = String::new();
        for i in 0..i {
            write!(&mut prepare, "{} q[{}];\n", gates[i as usize], i).unwrap();
        }
        let circuit = circuit.replace("%PREPARE%", &prepare);


        let mut half_cir = String::new();
        for _ in 0..j {
            for i in 0..i {
                write!(&mut half_cir, "{} q[{}];\n", c_gates[i as usize], i).unwrap();
            }
            write!(&mut half_cir, "\n").unwrap();
        }
        let circuit = circuit.replace("%HALF_DEPTH_CIRCUIT%", &half_cir);


        let mut half_cir_inv = String::new();
        for _ in 0..j {
            for i in 0..i {
                write!(&mut half_cir_inv, "{} q[{}];\n", c_gates_inv[i as usize], i).unwrap();
            }
            write!(&mut half_cir_inv, "\n").unwrap();
        }
        let circuit = circuit.replace("%HALF_DEPTH_INVERSE_CIRCUIT%", &half_cir_inv);


        let mut prepare_inv = String::new();
        for i in 0..i {
            write!(&mut prepare_inv, "{} q[{}];\n", gates[i as usize], i).unwrap();
        }
        let circuit = circuit.replace("%INVERT_PREPARE%", &prepare_inv);

        println!("{circuit}");

        Ok(Some(circuit))
    }
}
