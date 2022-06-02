use std::io::{BufWriter, Write};
use std::rc::Rc;
use std::sync::Mutex;

use fehler::throws;
use openqasm::{GateWriter, Symbol, Value};
use snafu::OptionExt;

use crate::error::OutOfBounds;

/// Translates gates to a primitive ones and outputs them
pub struct GatePrinter {
    buf: Rc<Mutex<BufWriter<Vec<u8>>>>,
    qreg: Vec<Symbol>,
    creg: Vec<Symbol>,
}

impl GatePrinter {
    pub fn new(buf: Rc<Mutex<BufWriter<Vec<u8>>>>) -> Self {
        Self {
            buf,
            qreg: vec![],
            creg: vec![],
        }
    }
}

// TODO can be used to parsed output
impl GateWriter for GatePrinter {
    type Error = crate::Error;

    #[throws(Self::Error)]
    fn initialize(&mut self, qreg: &[Symbol], creg: &[Symbol]) {
        self.qreg = qreg.to_vec();
        self.creg = creg.to_vec();
    }

    #[throws(Self::Error)]
    fn write_cx(&mut self, copy: usize, xor: usize) {
        let copy = self.qreg.get(copy).context(OutOfBounds)?;
        let xor = self.qreg.get(xor).context(OutOfBounds)?;

        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write_all(format!("cx {copy}, {xor};\n").as_bytes())?;
    }

    #[throws(Self::Error)]
    fn write_u(&mut self, theta: Value, phi: Value, lambda: Value, reg: usize) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write_all(format!("u({theta}, {phi}, {lambda}) {reg}\n").as_bytes())?;
    }

    #[throws(Self::Error)]
    fn write_opaque(&mut self, name: &Symbol, _: &[Value], _: &[usize]) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write_all(format!("opaque gate {name}\n").as_bytes())?;
    }

    #[throws(Self::Error)]
    fn write_barrier(&mut self, _: &[usize]) {
        // TODO
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write_all("\n".as_bytes())?;
    }

    #[throws(Self::Error)]
    fn write_measure(&mut self, from: usize, to: usize) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write_all(format!("measure {from} -> {to}\n").as_bytes())?;
    }

    #[throws(Self::Error)]
    fn write_reset(&mut self, reg: usize) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write_all(format!("reset {reg}\n").as_bytes())?;
    }

    #[throws(Self::Error)]
    fn start_conditional(&mut self, reg: usize, count: usize, value: u64) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write_all(format!("if ({reg}:{count} == {value}) {{\n").as_bytes())?;
    }

    #[throws(Self::Error)]
    fn end_conditional(&mut self) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write_all("}}\n".as_bytes())?;
    }
}
