use async_trait::async_trait;

use crate::args::types::CircuitBenchType;
use crate::args::CliArgsCircuit;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema};
use crate::lang_schemas::LangCircuit;
use crate::{Chooser, Error};

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
    async fn generate(&mut self, _: i32, _: i32, _: i32) -> Result<Option<GenCircuit>, Error> {
        use LangGateType::*;
        let mut gates = vec![
            LangGate::builder().t(X).i(0).build(),
            LangGate::builder().t(H).i(0).build(),
            LangGate::builder().t(S).i(1).build(),
        ];

        if !matches!(self.args.bench, CircuitBenchType::None) {
            gates.extend_from_slice(&[
                LangGate::builder().t(Sdg).i(1).build(),
                LangGate::builder().t(H).i(0).build(),
                LangGate::builder().t(X).i(0).build(),
            ]);
        }

        // Add NOT gate when should change init state
        if self.args.init_one {
            let mut tmp_gates = vec![];
            for i in 0..2 {
                tmp_gates.push(LangGate::builder().t(X).i(i).build());
            }
            tmp_gates.push(LangGate::builder().t(Barrier).i(-1).build());
            tmp_gates.extend(gates);
            gates = tmp_gates;
        }

        let c = LangCircuit::builder().gates(gates).width(2).build();
        Ok(Some(Chooser::get_lang_schema(self.args.schema).as_string(c).await?))
    }
}
