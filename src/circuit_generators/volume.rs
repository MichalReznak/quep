use async_trait::async_trait;
use itertools::interleave;

use crate::args::CliArgsCircuit;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema};
use crate::lang_schemas::LangCircuit;
use crate::{Chooser, Error};
use crate::args::types::CircuitBenchType;

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

            use CircuitBenchType::*;
            // TODO clean up
            match self.args.bench {
                Mirror => {
                    // TODO interleave with barriers??
                    result.push(LangGate::builder().t(Barrier).i(-1).build());
                    result.extend(inv_result.into_iter());
                }
                Cycle => {
                    // Pad vec with dummy gates
                    let mut oqs_gates2 = vec![];
                    for gate in &result {
                        oqs_gates2.push(gate.clone());
                        if gate.other.is_some() {
                            oqs_gates2.push(LangGate::builder().t(Dummy).i(-1).build())
                        }
                    }

                    let mut oqs_inv_gates2 = vec![];
                    for gate in &inv_result {
                        oqs_inv_gates2.push(gate.clone());
                        if gate.other.is_some() {
                            oqs_inv_gates2.push(LangGate::builder().t(Dummy).i(-1).build())
                        }
                    }

                    // TODO not pretty
                    let gates = oqs_gates2.chunks(i as usize).map(|e| e.clone())
                        .map(|e| e.into_iter().collect::<Vec<_>>()).collect::<Vec<_>>();

                    let inv_gates = oqs_inv_gates2.chunks(i as usize).map(|e| e.clone())
                        .map(|e| e.into_iter().rev().collect::<Vec<_>>()).collect::<Vec<_>>();

                    result = interleave(gates, inv_gates)
                        .flatten()
                        .map(|e| e.clone())
                        .collect::<Vec<_>>()
                        .chunks((4 * i) as usize)
                        .map(|e| e.clone())
                        .map(|e| e.into_iter().collect::<Vec<_>>())
                        .intersperse(vec![&LangGate::builder().t(Barrier).i(-1).build()])
                        .flatten().map(|e| e.clone()).filter(|e| !matches!(e.t, Dummy)).collect::<Vec<_>>();
                }
            }
        }

        let c = LangCircuit::builder().gates(result).width(i).build();
        Ok(Some(Chooser::get_lang_schema(self.args.schema).as_string(c).
        await?))
    }
}
