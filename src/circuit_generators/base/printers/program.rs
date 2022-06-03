use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{BufWriter, Write};

use fehler::throws;
use openqasm::{Expr, ProgramVisitor, Reg, Span, Stmt, Symbol};
use snafu::OptionExt;

use crate::error::OutOfBounds;
use crate::Error;

/// Outputs gates in the original format
pub struct ProgramPrinter {
    buf: BufWriter<Vec<u8>>,
    inverse_gates: Option<HashMap<String, String>>,
}

impl ProgramPrinter {
    pub fn new() -> Self {
        Self {
            buf: BufWriter::new(Vec::new()),
            inverse_gates: None,
        }
    }

    pub fn with_gates(inverse_gates: HashMap<String, String>) -> Self {
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
    type Error = Error;

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
        let args: Vec<_> = (*args)
            .iter()
            .map(|e| &**e)
            .map(|e| format!("{}[{}]", e.name, e.index.unwrap()))
            .collect();

        let (last, args) = args.split_last().context(OutOfBounds)?;
        let args: Vec<_> = args.iter().map(|e| format!("{e}, ")).collect();

        let mut a = String::new();
        for arg in args {
            a += &arg;
        }

        write!(&mut a, "{last};")?;

        let name = (&**name).as_str();

        let name = if let Some(ref gates) = self.inverse_gates {
            gates.get(&name.to_string()).unwrap_or(&name.to_string()).to_string()
        }
        else {
            name.to_string()
        };

        self.buf.write_all(format!("{} {}\n", &name, a).as_bytes())?;
    }
}
