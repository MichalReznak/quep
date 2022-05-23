use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::rc::Rc;
use std::sync::Mutex;

use collection_literals::collection;
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
    inverse_gates: Option<HashMap<&'static str, &'static str>>,
}

impl ProgramPrinter {
    pub fn new() -> Self {
        Self {
            buf: BufWriter::new(Vec::new()),
            inverse_gates: None,
        }
    }

    pub fn with_gates(inverse_gates: HashMap<&'static str, &'static str>) -> Self {
        Self {
            buf: BufWriter::new(Vec::new()),
            inverse_gates: Some(inverse_gates),
        }
    }

    pub fn result(&mut self) -> String {
        println!("{:?}", self.buf.buffer().to_vec());
        dbg!(String::from_utf8(self.buf.buffer().to_vec()).unwrap())
    }
}

impl ProgramVisitor for ProgramPrinter {
    type Error = std::convert::Infallible;

    #[throws(Self::Error)]
    fn visit_gate_def(
        &mut self,
        _name: &Span<Symbol>,
        _params: &[Span<Symbol>],
        _args: &[Span<Symbol>],
        _body: &[Span<Stmt>],
    ) {
        // ignore definitions
    }

    #[throws(Self::Error)]
    fn visit_gate(&mut self, name: &Span<Symbol>, _params: &[Span<Expr>], args: &[Span<Reg>]) {
        // println!("{name:?}: {args:?}");

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

        // println!("{a}");

        let name = (&**name).as_str();

        let name = if let Some(ref gates) = self.inverse_gates {
            gates.get(&name).unwrap_or_else(|| &name)
        }
        else {
            name
        };

        self.buf.write(format!("{} {}\n", &name, a).as_bytes()).unwrap();
        // println!("{:#?}", self.buf);
    }
}

pub struct ProgramParser {
    depth: i32,
    pub counts: HashMap<i32, i32>,
}

impl ProgramParser {
    pub fn new(depth: i32) -> Self {
        Self {
            depth,
            counts: HashMap::new(),
        }
    }
}

impl ProgramVisitor for ProgramParser {
    type Error = std::convert::Infallible;

    #[throws(Self::Error)]
    fn visit_gate_def(
        &mut self,
        _name: &Span<Symbol>,
        _params: &[Span<Symbol>],
        _args: &[Span<Symbol>],
        _body: &[Span<Stmt>],
    ) {
        // ignore definitions
    }

    #[throws(Self::Error)]
    fn visit_gate(&mut self, name: &Span<Symbol>, params: &[Span<Expr>], args: &[Span<Reg>]) {
        println!("{name:?}: {args:?}, {params:#?}");

        let args: Vec<_> = (*args).iter().map(|e| &**e).collect();

        let mut inserts = collection! { HashMap<i32, i32> };

        // TODO do not depend on index
        for arg in args {
            let i = arg.index.unwrap() as i32;
            match inserts.get_mut(&i) {
                None => {
                    inserts.insert(i, 1);
                }
                Some(val) => {
                    let mut a = *val;
                    a += 1;
                    inserts.insert(i, a);
                }
            };
        }

        let any = inserts.iter().any(|(k, _v)| {
            let counts = if let Some(a) = self.counts.get(k) {
                *a
            }
            else {
                0
            };

            let inserts_count = if let Some(a) = inserts.get(k) { *a } else { 0 };

            counts + inserts_count <= self.depth // TODO <= or < ??
        });
        if !any {}
    }
}
