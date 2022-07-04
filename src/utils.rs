use std::collections::HashMap;
use std::time::Duration;

use fehler::throws;
use itertools::interleave;
use log::info;
use pyo3::types::{PyDict, PyList};
use pyo3::{PyObject, Python};
use snafu::OptionExt;

use crate::error::Utf16;
use crate::ext::outputer::OutValue;
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

    let gates = oqs_gates2.chunks(i as usize).map(|e| e.to_vec());
    let inv_gates = oqs_inv_gates2.chunks(i as usize).map(|e| e.iter().rev().cloned().collect());
    let result = interleave(gates, inv_gates).flatten().collect::<Vec<_>>();

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

#[throws]
pub fn oqs_parse_circuit(
    oqs_gates: Vec<LangGate>,
    depth: i32,
    width: i32,
    mut iter: i32,
) -> (Vec<LangGate>, Vec<LangGate>) {
    let mut counts = HashMap::<i32, i32>::new();
    let mut gates = vec![];
    let mut inv_gates = vec![];

    for gate in &oqs_gates {
        if matches!(gate.t, LangGateType::Barrier) && gate.i < width {
            // Skip first N matches
            if iter > 0 {
                iter -= 1;
                continue;
            }

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

            // Skip first N matches
            if iter > 0 {
                iter -= 1;
                continue;
            }

            gates.push(gate.clone());
            inv_gates.push(gate.inverse());
        }
    }

    inv_gates.reverse();
    (gates, inv_gates)
}

#[throws]
pub fn filter_incorrect_values(values: Vec<OutValue>) -> OutValue {
    // Get count of occurrences
    let mut m: HashMap<String, usize> = HashMap::new();
    for value in &values {
        *m.entry(value.result.clone()).or_default() += 1;
    }

    // Get highest
    let mm = dbg!(m.clone());
    let key = dbg!(m.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k).unwrap());
    let c = dbg!(*mm.get(&key.clone()).unwrap() as i32);

    // Filter out
    let mut value = dbg!(dbg!(values.into_iter().filter(|e| e.result == key)).reduce(|acc, e| {
        OutValue::builder()
            .result(acc.result)
            .correct(acc.correct + e.correct)
            .is_correct(true)
            .build()
    }))
    .unwrap();
    value.correct /= c;
    value
}
