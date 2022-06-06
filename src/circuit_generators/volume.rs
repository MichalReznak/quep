use async_trait::async_trait;

use crate::args::CliArgsCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema};
use crate::lang_schemas::LangCircuit;
use crate::{Chooser, Error};
use crate::ext::types::circuit_generator::GenCircuit;

#[allow(dead_code)]
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
    async fn generate(&mut self, i: i32, j: i32, _: i32) -> Result<Option<GenCircuit>, Error> {
        let i = i + 1;
        let j = j + 1;

        use LangGateType::*;
        let gates = vec![X, H, Z, Y];
        let mut result = vec![];

        for j in 0..j {
            for i in 0..i {
                let gate =
                    LangGate::builder().t(gates[(i + j) as usize % gates.len()]).i(i).build();
                result.push(gate);
            }
        }

        let mut inv_result = result.clone();
        inv_result.reverse();

        let c = LangCircuit::builder().gates(result).inv_gates(inv_result).width(i).build();
        Ok(Some(Chooser::get_lang_schema(self.args.schema).as_string(c).await?))
    }
}
