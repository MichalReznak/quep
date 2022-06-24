use fehler::throws;
use itertools::interleave;
use snafu::OptionExt;

use crate::error::Utf16;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::Error;

#[throws]
pub fn dir(s: &str) -> String {
    dunce::canonicalize(s)?.to_str().context(Utf16)?.to_owned()
}

#[cfg(not(debug_assertions))]
pub fn debug() -> bool {
    false
}

#[cfg(debug_assertions)]
pub fn debug() -> bool {
    true
}

pub fn cycle(gates: Vec<LangGate>, inv_gates: Vec<LangGate>, i: i32) -> Vec<LangGate> {
    // Pad vec with dummy gates
    let mut oqs_gates2 = vec![];
    for gate in &gates {
        oqs_gates2.push(gate.clone());
        if gate.other.is_some() {
            oqs_gates2.push(LangGate::builder().t(LangGateType::Dummy).i(-1).build())
        }
    }

    let mut oqs_inv_gates2 = vec![];
    for gate in &inv_gates {
        oqs_inv_gates2.push(gate.clone());
        if gate.other.is_some() {
            oqs_inv_gates2.push(LangGate::builder().t(LangGateType::Dummy).i(-1).build())
        }
    }

    // TODO not pretty
    let gates = oqs_gates2.chunks(i as usize).map(|e| e.to_vec());

    let inv_gates = oqs_inv_gates2.chunks(i as usize).map(|e| e.iter().rev().cloned().collect());

    let result = interleave(gates, inv_gates).flatten().collect::<Vec<_>>();

    // TODO should be just "i" instead of "2 * i" for Volume
    result
        .chunks((2 * i) as usize)
        .intersperse(&[LangGate::builder().t(LangGateType::Barrier).i(-1).build()])
        .flatten()
        .filter(|e| !matches!(e.t, LangGateType::Dummy))
        .cloned()
        .collect::<Vec<_>>()
}
