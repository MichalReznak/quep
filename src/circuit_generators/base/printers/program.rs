use std::collections::{HashMap, HashSet};
use std::io::{BufWriter, Write};

use fehler::throws;
use openqasm::{Expr, ProgramVisitor, Reg, Span, Stmt, Symbol};

/// Outputs gates in the original format
pub struct ProgramPrinter {
    buf: BufWriter<Vec<u8>>,
    inverse_gates: Option<HashMap<&'static str, &'static str>>,
    pub current_gate_i: i32,
    len: i32,
    pub included_gates: HashSet<i32>,
}

impl ProgramPrinter {
    pub fn new(included_gates: HashSet<i32>) -> Self {
        Self {
            buf: BufWriter::new(Vec::new()),
            inverse_gates: None,
            current_gate_i: 0,
            len: 0,
            included_gates,
        }
    }

    pub fn with_gates(
        included_gates: HashSet<i32>,
        inverse_gates: HashMap<&'static str, &'static str>,
        len: i32,
    ) -> Self {
        println!("LEn: {len}");
        Self {
            buf: BufWriter::new(Vec::new()),
            inverse_gates: Some(inverse_gates),
            current_gate_i: 0,
            len,
            included_gates,
        }
    }

    pub fn result(&mut self) -> String {
        String::from_utf8(self.buf.buffer().to_vec()).unwrap()
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
        let i = if let Some(_) = self.inverse_gates {
            self.len - self.current_gate_i
        }
        else {
            self.current_gate_i
        };
        self.current_gate_i += 1;

        if !self.included_gates.contains(&i) {
            return;
        }

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

        let name = (&**name).as_str();

        let name = if let Some(ref gates) = self.inverse_gates {
            gates.get(&name).unwrap_or_else(|| &name)
        }
        else {
            name
        };

        self.buf.write(format!("{} {}\n", &name, a).as_bytes()).unwrap();
    }
}
