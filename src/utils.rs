use std::time::Duration;

use fehler::throws;
use itertools::interleave;
use log::info;
use pyo3::types::{PyDict, PyList};
use pyo3::{PyObject, Python};
use snafu::OptionExt;

use crate::error::Utf16;
use crate::ext::types::lang_schema::LangGateType::{Barrier, X};
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::types::MetaInfo;
use crate::Error::PyDowncastError;
use crate::{CircuitBenchType, Error};

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

pub fn inverse(
    bench: CircuitBenchType,
    mut gates: Vec<LangGate>,
    mut gates_inv: Vec<LangGate>,
    i: i32,
) -> Vec<LangGate> {
    use CircuitBenchType::*;

    match bench {
        Mirror => {
            // TODO interleave with barriers??
            gates_inv.reverse();
            gates.push(LangGate::builder().t(Barrier).i(-1).build());
            gates.extend(gates_inv.into_iter());
            gates
        }
        Cycle => cycle(gates, gates_inv, 2 * i),
        None => gates,
    }
}

pub fn init_one(oqs_gates: Vec<LangGate>, i: i32) -> Vec<LangGate> {
    let mut gates = vec![];
    for i in 0..i {
        gates.push(LangGate::builder().t(X).i(i).build());
    }
    gates.push(LangGate::builder().t(Barrier).i(-1).build());
    gates.extend(oqs_gates);
    gates
}

pub async fn provider_run(py_instance: &PyObject) -> Result<Vec<String>, Error> {
    let res = Python::with_gil(|py| -> Result<_, Error> {
        let fun = py_instance.call_method0(py, "run_all")?;

        // Change to vector
        // TODO better?
        let res: Vec<_> = if let Ok(list) = fun.cast_as::<PyList>(py) {
            list.into_iter().collect()
        }
        else {
            vec![fun.cast_as::<PyDict>(py).map_err(|_| PyDowncastError).unwrap()]
        };

        let res: Vec<_> = res
            .into_iter()
            .map(|e| e.cast_as::<PyDict>().map_err(|_| PyDowncastError).unwrap())
            .map(|e| {
                let mut highest = ("".to_string(), 0);
                for (key, val) in e.into_iter() {
                    info!("{key:#?}, {val:#?}");
                    let val: i32 = val.extract().unwrap();

                    if val > highest.1 {
                        highest = (key.to_string(), val);
                    }
                }
                format!("{}: {}", highest.0, highest.1)
            })
            .collect();
        Ok(res)
    })?;

    Python::with_gil(|py| -> Result<_, Error> {
        py_instance.call_method0(py, "clear_circuits")?;
        Ok(())
    })?;

    Ok(res)
}

pub async fn provider_meta_info(py_instance: &PyObject) -> Result<MetaInfo, Error> {
    Python::with_gil(|py| -> Result<_, Error> {
        let res = py_instance.call_method0(py, "get_meta_info")?;
        let res = res.cast_as::<PyDict>(py).map_err(|_| PyDowncastError).unwrap();

        let time: f64 = res.get_item("time").unwrap().extract()?;
        let time = Duration::from_secs_f64(time);

        Ok(MetaInfo::builder().time(time).build())
    })
}
