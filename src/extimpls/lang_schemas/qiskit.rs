use std::fmt::Write;
use std::path::Path;

use async_trait::async_trait;
use fehler::throws;
use regex::Regex;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use unwrap_infallible::UnwrapInfallible;

use crate::args::types::LangSchemaType;
use crate::args::CliArgsLangSchema;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::LangSchema;
use crate::lang_schemas::LangCircuit;
use crate::Error;

// TODO do it gate independent
const CIRCUIT_TEMPLATE: &str = r#"
from qiskit import *

circ = QuantumCircuit(%WIDTH%, %WIDTH%)

%RESET%

circ.barrier()

%GATES%

circ.barrier()

%MEASURE%
"#;

// TODO add all measures

// TODO remove fixed reg name
fn gate_to_string(gate: &LangGate) -> String {
    use LangGateType::*;
    match gate.t {
        Id => format!("circ.id({})", gate.i),
        X => format!("circ.x({})", gate.i),
        Y => format!("circ.y({})", gate.i),
        Z => format!("circ.z({})", gate.i),
        H => format!("circ.h({})", gate.i),
        S => format!("circ.s({})", gate.i),
        Sdg => format!("circ.sdg({})", gate.i),
        Cx => format!("circ.cx({}, {})", gate.i, gate.other.unwrap()),
        Cz => format!("circ.cz({}, {})", gate.i, gate.other.unwrap()),
        Swap => format!("circ.swap({}, {})", gate.i, gate.other.unwrap()),
        Barrier => {
            if gate.i >= 0 {
                format!("circ.barrier({})", gate.i)
            }
            else {
                "circ.barrier()".to_string()
            }
        }
        _ => panic!("Never"),
    }
}

fn string_to_gate(str: &str) -> Option<LangGateType> {
    use LangGateType::*;
    match str {
        "id" => Some(Id),
        "x" => Some(X),
        "y" => Some(Y),
        "z" => Some(Z),
        "h" => Some(H),
        "s" => Some(S),
        "sdg" => Some(Sdg),
        "cx" => Some(Cx),
        "cz" => Some(Cz),
        "swap" => Some(Swap),
        "barrier" => Some(Barrier),
        val => {
            println!("{val}");
            None
        }
    }
}

#[derive(Default)]
pub struct QiskitSchema {
    pub gates: Vec<LangGate>,
}

impl QiskitSchema {
    #[throws]
    pub fn from_args(_: &CliArgsLangSchema) -> Self {
        Self { gates: vec![] }
    }
}

#[async_trait]
impl LangSchema for QiskitSchema {
    async fn parse_file(&self, path: &str) -> Result<Vec<LangGate>, Error> {
        let re =
            Regex::new(r"circ\.(?P<name>[a-zA-Z0-9]+)\((?P<index>\d+)(,\s*(?P<other>\d+))*\)")?;

        let file = File::open(Path::new(&path)).await?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut gates = vec![];

        while let Some(l) = lines.next_line().await? {
            let c = re.captures(&l);
            if c.is_none() {
                continue;
            };
            let c = c.unwrap();

            let name = c["name"].parse::<String>().unwrap_infallible();
            let i = c["index"].parse::<i32>().unwrap();

            let other = c.name("other");

            if let Some(gate_type) = string_to_gate(&name) {
                let gate = if let Some(o) = other {
                    LangGate::builder()
                        .t(gate_type)
                        .i(i)
                        .other(o.as_str().parse::<i32>().unwrap())
                        .build()
                }
                else {
                    LangGate::builder().t(gate_type).i(i).build()
                };

                gates.push(gate);
            }
        }

        Ok(gates)
    }

    async fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error> {
        // Add width
        let res = CIRCUIT_TEMPLATE.replace("%WIDTH%", &circ.width.to_string());

        // Add resets
        let mut resets = String::new();
        for i in 0..circ.width {
            writeln!(&mut resets, "circ.reset({})", i)?;
        }
        let res = res.replace("%RESET%", &resets);

        // Add gates
        let mut gates = String::new();
        for gate in circ.gates {
            writeln!(&mut gates, "{}", gate_to_string(&gate))?;
        }
        let res = res.replace("%GATES%", &gates);

        // Add measurements
        let mut measures = String::new();
        for i in 0..circ.width {
            writeln!(&mut measures, "circ.measure({}, {})", i, i)?;
        }
        let res = res.replace("%MEASURE%", &measures);

        println!("{res}");

        Ok(GenCircuit::builder().circuit(res).t(LangSchemaType::Qiskit).build())
    }
}
