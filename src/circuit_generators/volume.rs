use std::fmt::Write;
use std::path::Path;

use async_trait::async_trait;
use derive_more::Constructor;
use openqasm as oq;
use oq::GenericError;

use crate::traits::CircuitGenerator;
use crate::Error;

const CIRCUIT_PLACEHOLDER: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[%SIZE%];
creg c[%SIZE%];

%RESET%
%DEPTH%

measure q -> c;
"#;

#[derive(Constructor)]
pub struct VolumeCircuitGenerator;

#[async_trait]
impl CircuitGenerator for VolumeCircuitGenerator {
    async fn generate(&self, i: i32) -> Result<Option<String>, Error> {
        if i > 5 {
            Ok(None)
        }
        else {
            let i = i + 1;
            let circuit = CIRCUIT_PLACEHOLDER.replace("%SIZE%", &i.to_string());

            let mut resets = String::new();
            for i in 0..i {
                write!(&mut resets, "reset q[{}];\n", i).unwrap();
            }
            let circuit = circuit.replace("%RESET%", &resets);

            let mut depth = String::new();
            for _ in 0..i {
                for i in 0..i {
                    write!(&mut depth, "x q[{}];\n", i).unwrap();
                }
                write!(&mut depth, "\n").unwrap();
            }
            let circuit = circuit.replace("%DEPTH%", &depth);

            println!("{circuit}");
            let check: Result<_, oq::Errors> = try {
                let mut cache = oq::SourceCache::new();
                let mut parser = oq::Parser::new(&mut cache);
                parser.parse_source(circuit.to_string(), Some(&Path::new(".")));
                parser.done().to_errors()?.type_check().to_errors()?;
            };
            if let Err(errors) = check {
                println!("{errors:#?}");
                Err(crate::Error::SomeError)
            }
            else {
                Ok(Some(circuit.to_string()))
            }
        }
    }
}
