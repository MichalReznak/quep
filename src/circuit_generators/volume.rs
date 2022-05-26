use std::fmt::Write;
use std::path::Path;

use async_trait::async_trait;

use openqasm as oq;
use oq::GenericError;

use crate::args::CliArgsCircuit;
use crate::traits::CircuitGenerator;
use crate::Error;

const CIRCUIT_PLACEHOLDER: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[%SIZE%];
creg c[%SIZE%];

%RESET%
%DEPTH%

barrier q;

measure q -> c;
"#;

pub struct VolumeCircuitGenerator {
    args: CliArgsCircuit,
}

impl VolumeCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for VolumeCircuitGenerator {
    async fn generate(&mut self, i: i32, j: i32, _: i32, _: bool) -> Result<Option<String>, Error> {
        let gates = vec!["x", "h", "z", "y"];

        let i = i + 1;
        let j = j + 1;
        let circuit = CIRCUIT_PLACEHOLDER.replace("%SIZE%", &i.to_string());

        let mut resets = String::new();
        for i in 0..i {
            write!(&mut resets, "reset q[{}];\n", i)?;
        }
        let circuit = circuit.replace("%RESET%", &resets);

        let mut depth = String::new();
        for j in 0..j {
            for i in 0..i {
                write!(&mut depth, "{} q[{}];\n", gates[(i + j) as usize % gates.len()], i)?;
            }
            write!(&mut depth, "\n")?;
        }
        let circuit = circuit.replace("%DEPTH%", &depth);

        let mut cache = oq::SourceCache::new();
        let mut parser = oq::Parser::new(&mut cache);

        let check: Result<_, oq::Errors> = try {
            parser.parse_source(circuit.to_string(), Some(&Path::new(".")));
            parser.done().to_errors()?.type_check().to_errors()?;
        };
        if let Err(errors) = check {
            errors.print(&mut cache)?;
            Err(crate::Error::SomeError)
        }
        else {
            Ok(Some(circuit.to_string()))
        }
    }
}
