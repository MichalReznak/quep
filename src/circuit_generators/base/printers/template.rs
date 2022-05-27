use std::collections::{HashMap, HashSet};
use std::io::{BufWriter, Write};

use fehler::throws;
use openqasm::{Decl, Expr, Program, ProgramVisitor, Reg, Span, Stmt, Symbol};

/// Outputs template in the original format
pub struct TemplatePrinter {
    buf: BufWriter<Vec<u8>>,
}

impl TemplatePrinter {
    pub fn new() -> Self {
        Self {
            buf: BufWriter::new(Vec::new()),
        }
    }

    pub fn result(&mut self) -> String {
        // TODO remove fixed format definition
        "OPENQASM 2.0;\n".to_owned() + &String::from_utf8(self.buf.buffer().to_vec()).unwrap()
    }
}

impl ProgramVisitor for TemplatePrinter {
    type Error = std::convert::Infallible;

    #[throws(Self::Error)]
    fn visit_qreg(&mut self, reg: &Span<Reg>) {
        let reg = &*reg.inner;
        let name = reg.name.as_str();
        // TODO can fail
        let i = reg.index.unwrap_or(0);

        self.buf.write(format!("qreg {name}[{i}];\n").as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn visit_creg(&mut self, reg: &Span<Reg>) {
        let reg = &*reg.inner;
        let name = reg.name.as_str();
        let i = reg.index.unwrap_or(0);

        self.buf.write(format!("creg {name}[{i}];\n").as_bytes()).unwrap();
    }

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
        let (last, args) = args.split_last().unwrap();
        let args: Vec<_> = args.into_iter().map(|e| format!("{e}, ")).collect();

        let mut a = String::new();
        for arg in args {
            a += &arg;
        }
        a += &format!("{last};");

        self.buf.write(format!("{} {}\n", (&**name), a).as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn visit_include(&mut self, file: &Span<Symbol>) {
        let name = &*file.inner.as_str();
        self.buf.write(format!("include \"{name}\";\n").as_bytes()).unwrap();
    }

    #[throws(Self::Error)]
    fn visit_measure(&mut self, from: &Span<Reg>, to: &Span<Reg>) {
        let from_name = &*from.inner.name.as_str();
        let to_name = &*to.inner.name.as_str();

        // TODO support index
        self.buf
            .write(format!("measure {from_name} -> {to_name};\n").as_bytes())
            .unwrap();
    }

    // TODO define all other types
}
