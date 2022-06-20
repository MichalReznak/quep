use std::collections::HashMap;

use async_trait::async_trait;
use fehler::throws;

use crate::args::CliArgsCircuit;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::{Chooser, Error};

#[throws]
fn oqs_parse_circuit(
    oqs: &LangSchemaDyn,
    depth: i32,
    width: i32,
) -> (Vec<LangGate>, Vec<LangGate>) {
    let mut counts = HashMap::<i32, i32>::new();
    let mut gates = vec![];
    let mut inv_gates = vec![];

    for gate in &oqs.get_gates() {
        if matches!(gate.t, LangGateType::Barrier) && gate.i < width {
            gates.push(gate.clone());
            inv_gates.push(gate.inverse());
            continue;
        }

        let mut second_ok = true;

        let count = if let Some(c) = counts.get_mut(&gate.i) {
            *c
        }
        else {
            0
        };

        let first_ok = count < depth && gate.i < width;

        use LangGateType::*;
        match gate.t {
            Cx | Cz | Swap => {
                let count = if let Some(c) = counts.get_mut(&gate.other.unwrap()) {
                    *c
                }
                else {
                    0
                };

                second_ok = count < depth && gate.other.unwrap() < width;
            }
            _ => {}
        }

        // Do all indices fulfill the limit?
        if first_ok && second_ok {
            // Add limits for the next gate
            if let Some(c) = counts.get_mut(&gate.i) {
                *c += 1;
            }
            else {
                counts.insert(gate.i, 1);
            }

            match gate.t {
                Cx | Cz | Swap => {
                    if let Some(c) = counts.get_mut(&gate.other.unwrap()) {
                        *c += 1;
                    }
                    else {
                        counts.insert(gate.other.unwrap(), 1);
                    }
                }
                _ => {}
            }

            gates.push(gate.clone());
            inv_gates.push(gate.inverse());
        }
    }

    inv_gates.reverse();
    (gates, inv_gates)
}

// NOTES:
// order of reg defines order in circuit
// barrier inputs I-gates to unset ones
// Now restricted version -> single qreg[n] and creg[n]
//      Measure is done on all
//      They define only the circuit
//      Number of qubits does not change
// Non inverse gates are not handled
// For now only a single qreg and creg can be defined
// Gates needs to be defined only in an entry file (otherwise order is wrong)
// Depth is number of gates on each qubit it's not with automatic identity gates

#[allow(dead_code)]
#[derive(Debug)]
pub struct BaseCircuitGenerator {
    args: CliArgsCircuit,
}

impl BaseCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(
        &mut self,
        depth: i32,
        width: i32,
        _iter: i32, // TODO
        mirror: bool,
    ) -> Result<Option<GenCircuit>, Error> {
        let depth = depth + 1;
        let width = width + 1;
        // TODO check circuit size
        // TODO barriers support
        // TODO different order of operations

        let mut lang_schema = Chooser::get_lang_schema(self.args.schema);

        lang_schema.parse_file(&self.args.source).await?;

        let (mut gates, inv_gates) = oqs_parse_circuit(&lang_schema, depth, width)?;

        if mirror {
            gates.push(LangGate::builder().t(LangGateType::Barrier).i(-1).build());
            gates.extend(inv_gates.into_iter());
        }

        let lang_circuit =
            LangCircuit::builder().width(width).gates(gates).build();
        let circuit = lang_schema.as_string(lang_circuit).await?;
        Ok(Some(circuit))
    }
}
