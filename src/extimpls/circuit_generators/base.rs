//! NOTES:
//! order of reg defines order in circuit
//! barrier inputs I-gates to unset ones
//! Now restricted version -> single qreg[n] and creg[n]
//!      Measure is done on all
//!      They define only the circuit
//!      Number of qubits does not change
//! Non inverse gates are not handled
//! For now only a single qreg and creg can be defined
//! Gates needs to be defined only in an entry file (otherwise order is wrong)
//! Depth is number of gates on each qubit it's not with automatic identity
//! gates
//! Iter: How many valid gates should be skipped

use async_trait::async_trait;
use fehler::throws;
use itertools::interleave;

use crate::args::types::CircuitBenchType;
use crate::args::CliArgsCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::utils::oqs_parse_circuit;
use crate::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub struct BaseCircuitGenerator {
    args: CliArgsCircuit,
}

impl BaseCircuitGenerator {
    #[throws]
    pub fn from_args(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(
        &mut self,
        lang_schema: &LangSchemaDyn,
        width: i32,
        depth: i32,
        iter: i32,
    ) -> Result<Option<LangCircuit>, Error> {
        let oqs_gates = lang_schema.parse_file(&self.args.source)?.gates;

        let (mut oqs_gates, mut inv_gates) = oqs_parse_circuit(oqs_gates, depth, width, iter)?;

        use CircuitBenchType::*;

        match self.args.bench {
            Mirror => {
                oqs_gates.push(LangGate::builder().t(LangGateType::Barrier).i(-1).build());
                oqs_gates.extend(inv_gates.into_iter());
            }
            Cycle => {
                inv_gates.reverse();
                oqs_gates = interleave(oqs_gates, inv_gates).collect();
            }
            None => {}
        }

        // Add NOT gate when should change init state
        if self.args.init_one {
            let mut gates = vec![];
            for i in 0..width {
                gates.push(LangGate::builder().t(LangGateType::X).i(i).build());
            }
            gates.push(LangGate::builder().t(LangGateType::Barrier).i(-1).build());
            gates.extend(oqs_gates);
            oqs_gates = gates;
        }

        Ok(Some(
            LangCircuit::builder()
                .creg(width)
                .qreg(width)
                .gates(oqs_gates)
                .build(),
        ))
    }
}
