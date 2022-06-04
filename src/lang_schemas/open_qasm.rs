use std::fmt::Write;

use async_trait::async_trait;
use derive_more::Constructor;

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
            writeln!(&mut gates, "{}", gate.to_string())?;
        }
        let res = res.replace("%GATES%", &gates);

        // Add inverse gates
        let mut gates = String::new();
        for gate in circ.inv_gates {
            writeln!(&mut gates, "{}", gate.to_string())?;
        }
        let res = res.replace("%INV_GATES%", &gates);

        Ok(res)
    }
}
