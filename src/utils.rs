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
    let gates = oqs_gates2
        .chunks(i as usize)
        .map(|e| e.clone())
        .map(|e| e.into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let inv_gates = oqs_inv_gates2
        .chunks(i as usize)
        .map(|e| e.clone())
        .map(|e| e.into_iter().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    interleave(gates, inv_gates)
        .flatten()
        .map(|e| e.clone())
        .collect::<Vec<_>>()
        .chunks((4 * i) as usize)
        .map(|e| e.clone())
        .map(|e| e.into_iter().collect::<Vec<_>>())
        .intersperse(vec![&LangGate::builder().t(LangGateType::Barrier).i(-1).build()])
        .flatten()
        .map(|e| e.clone())
        .filter(|e| !matches!(e.t, LangGateType::Dummy))
        .collect::<Vec<_>>()
}
