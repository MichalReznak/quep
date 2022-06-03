use std::path::Path;

use async_trait::async_trait;

use crate::traits::CircuitGenerator;
use crate::Error;

const CIRCUIT: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

reset q[0];
x q[0];
h q[0];
s q[0];
sdg q[0];
h q[0];
x q[0];

barrier q;

measure q -> c;
"#;

use openqasm as oq;
use oq::GenericError;

use crate::args::CliArgsCircuit;

#[allow(dead_code)]
pub struct BasicCircuitGenerator {
    args: CliArgsCircuit,
}

#[allow(dead_code)]
impl BasicCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for BasicCircuitGenerator {
    async fn generate(&mut self, i: i32, j: i32, _: i32) -> Result<Option<String>, Error> {
        if i > 0 || j > 0 {
            Ok(None)
        }
        else {
            let mut cache = oq::SourceCache::new();
            let mut parser = oq::Parser::new(&mut cache);

            let check: Result<_, oq::Errors> = try {
                parser.parse_source(CIRCUIT.to_string(), Some(&Path::new(".")));
                parser.done().to_errors()?.type_check().to_errors()?;
            };
            if let Err(errors) = check {
                errors.print(&mut cache)?;
                Err(crate::Error::SomeError)
            }
            else {
                Ok(Some(CIRCUIT.to_string()))
            }
        }
    }
}
