use std::fmt::Write;

use async_trait::async_trait;
use derive_more::Constructor;

use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::LangSchema;
use crate::lang_schemas::LangCircuit;
use crate::Error;

// TODO do it gate independent
const CIRCUIT_TEMPLATE: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[%WIDTH%];
creg c[%WIDTH%];

%RESET%

barrier q;

%GATES%

barrier q;

%INV_GATES%

barrier q;

measure q -> c;
"#;

#[derive(Constructor)]
pub struct OpenQasmSchema;

// TODO remove fixed q
// TODO should not implement openqasm directly
fn gate_to_string(gate: &LangGate) -> String {
    use LangGateType::*;
    match gate.t {
        Id => format!("id q[{}];", gate.i),
        X => format!("x q[{}];", gate.i),
        Y => format!("y q[{}];", gate.i),
        Z => format!("z q[{}];", gate.i),
        H => format!("h q[{}];", gate.i),
        S => format!("s q[{}];", gate.i),
        Sdg => format!("sdg q[{}];", gate.i),
        Cx => format!("cx q[{}], q[{}];", gate.i, gate.other.unwrap()),
        Cz => format!("cz q[{}], q[{}];", gate.i, gate.other.unwrap()),
        Swap => format!("swap q[{}], q[{}];", gate.i, gate.other.unwrap()),
    }
}

#[async_trait]
impl LangSchema for OpenQasmSchema {
    async fn as_string(&mut self, circ: LangCircuit) -> Result<String, Error> {
        // Add width
        let res = CIRCUIT_TEMPLATE.replace("%WIDTH%", &circ.width.to_string());

        // Add resets
        let mut resets = String::new();
        for i in 0..circ.width {
            writeln!(&mut resets, "reset q[{}];", i)?;
        }
        let res = res.replace("%RESET%", &resets);

        // Add gates
        let mut gates = String::new();
        for gate in circ.gates {
            writeln!(&mut gates, "{}", gate_to_string(&gate))?;
        }
        let res = res.replace("%GATES%", &gates);

        // Add inverse gates
        let mut gates = String::new();
        for gate in circ.inv_gates {
            writeln!(&mut gates, "{}", gate_to_string(&gate))?;
        }
        let res = res.replace("%INV_GATES%", &gates);

        Ok(res)
    }
}
