use std::collections::{HashMap, HashSet};
use std::io::{BufWriter, Write};

use fehler::throws;
use openqasm::{Expr, ProgramVisitor, Reg, Span, Stmt, Symbol};
use snafu::OptionExt;

use crate::error::OutOfBounds;
use crate::Error;

/// Outputs gates in the original format
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

    #[throws]
    pub fn result(&mut self) -> String {
        String::from_utf8(self.buf.buffer().to_vec())?
    }
}

impl ProgramVisitor for ProgramPrinter {
    type Error = crate::Error;

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
        let args: Vec<_> = (*args).iter().map(|e| &**e).collect();

        let args: Vec<_> =
            args.into_iter().map(|e| format!("{}[{}]", e.name, e.index.unwrap())).collect();
        let (last, args) = args.split_last().context(OutOfBounds)?;
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

        self.buf.write(format!("{} {}\n", &name, a).as_bytes())?;
    }
}
