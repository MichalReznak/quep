use std::io::{BufWriter, Write};
use std::rc::Rc;
use std::sync::Mutex;

use fehler::throws;
use openqasm::{Expr, GateWriter, ProgramVisitor, Reg, Span, Stmt, Symbol, Value};

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
    type Error = std::convert::Infallible;

    #[throws(Self::Error)]
    fn initialize(&mut self, qreg: &[Symbol], creg: &[Symbol]) {
        self.qreg = qreg.to_vec();
        self.creg = creg.to_vec();
    }

    #[throws(Self::Error)]
    fn write_cx(&mut self, copy: usize, xor: usize) {
        let copy = self.qreg.get(copy).unwrap();
        let xor = self.qreg.get(xor).unwrap();

        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write(format!("cx {copy}, {xor};\n").as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn write_u(&mut self, theta: Value, phi: Value, lambda: Value, reg: usize) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write(format!("u({theta}, {phi}, {lambda}) {reg}\n").as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn write_opaque(&mut self, name: &Symbol, _: &[Value], _: &[usize]) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write(format!("opaque gate {name}\n").as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn write_barrier(&mut self, _: &[usize]) {
        // TODO
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write("\n".as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn write_measure(&mut self, from: usize, to: usize) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write(format!("measure {from} -> {to}\n").as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn write_reset(&mut self, reg: usize) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write(format!("reset {reg}\n").as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn start_conditional(&mut self, reg: usize, count: usize, value: u64) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write(format!("if ({reg}:{count} == {value}) {{\n").as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn end_conditional(&mut self) {
        let mut lock = self.buf.lock().unwrap();
        let buf = lock.get_mut();
        buf.write(format!("}}\n").as_bytes()).unwrap();
    }
}

pub struct ProgramPrinter {
    buf: BufWriter<Vec<u8>>,
}

impl ProgramPrinter {
    pub fn new() -> Self {
        Self {
            buf: BufWriter::new(Vec::new()),
        }
    }

    pub fn result(&mut self) -> String {
        println!("{:?}", self.buf.buffer().to_vec());
        dbg!(String::from_utf8(self.buf.buffer().to_vec()).unwrap())
    }
}

impl ProgramVisitor for ProgramPrinter {
    type Error = std::convert::Infallible;

    fn visit_gate_def(
        &mut self,
        _name: &Span<Symbol>,
        _params: &[Span<Symbol>],
        _args: &[Span<Symbol>],
        _body: &[Span<Stmt>],
    ) -> Result<(), Self::Error> {
        // ignore definitions
        Ok(())
    }

    fn visit_gate(
        &mut self,
        name: &Span<Symbol>,
        _params: &[Span<Expr>],
        args: &[Span<Reg>],
    ) -> Result<(), Self::Error> {
        println!("{name:?}: {args:?}");

        let args: Vec<_> = (*args).iter().map(|e| &**e).collect();

        let args: Vec<_> =
            args.into_iter().map(|e| format!("{}[{}]", e.name, e.index.unwrap())).collect();
        let (last, args) = args.split_last().unwrap();
        let args: Vec<_> = args.into_iter().map(|e| format!("{e}, ")).collect();

        let mut a = String::new();
        for arg in args {
            a += &arg;
        }
        a += &format!("{last};");

        println!("{a}");

        self.buf.write(format!("{} {}\n", &**name, a).as_bytes()).unwrap();
        println!("{:#?}", self.buf);
        Ok(())
    }
}
