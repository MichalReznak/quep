use async_trait::async_trait;

use crate::args::CliArgsCircuit;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema};
use crate::lang_schemas::LangCircuit;
use crate::{Chooser, Error};

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
    async fn generate(
        &mut self,
        i: i32,
        _: i32,
        _: i32,
        mirror: bool,
    ) -> Result<Option<GenCircuit>, Error> {
        let i = i + 1;
        let j = i / 2;

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

        if mirror {
            let mut inv_result = result.clone();
            inv_result.reverse();
            result.push(LangGate::builder().t(Barrier).i(-1).build());
            result.extend(inv_result.into_iter());
        }

        let c = LangCircuit::builder().gates(result).width(i).build();
        Ok(Some(Chooser::get_lang_schema(self.args.schema).as_string(c).
        await?))
    }
}
