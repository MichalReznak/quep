use std::io::{BufWriter, Write};

use fehler::throws;
use openqasm::{Decl, Program, ProgramVisitor, Reg, Span, Stmt, Symbol};

use crate::Error;

/// Outputs template in the original format
pub struct TemplatePrinter {
    buf: BufWriter<Vec<u8>>,
    width: i32,
}

impl TemplatePrinter {
    pub fn new(width: i32) -> Self {
        Self {
            width,
            buf: BufWriter::new(Vec::new()),
        }
    }

    #[throws]
    pub fn result(&mut self) -> String {
        // TODO remove fixed format definition
        "OPENQASM 2.0;\n".to_owned() + &String::from_utf8(self.buf.buffer().to_vec())?
    }
}

impl ProgramVisitor for TemplatePrinter {
    type Error = Error;

    #[throws(Self::Error)]
    fn walk_program(&mut self, program: &Program) {
        for decl in &program.decls {
            if matches!(&*decl.inner, Decl::Include { .. }) {
                self.visit_decl(decl)?;
            }
        }
        self.buf.write(format!("\n").as_bytes())?;

        for decl in &program.decls {
            let decl = decl;
            if matches!(&*decl.inner, Decl::Def { .. }) {
                self.visit_decl(decl)?;
            }
        }
        self.buf.write(format!("\n").as_bytes())?;

        for decl in &program.decls {
            if matches!(&*decl.inner, Decl::QReg { .. }) {
                self.visit_decl(decl)?;
            }
        }
        self.buf.write(format!("\n").as_bytes())?;

        for decl in &program.decls {
            if matches!(&*decl.inner, Decl::CReg { .. }) {
                self.visit_decl(decl)?;
            }
        }
        self.buf.write(format!("\n").as_bytes())?;

        // Replace statements with predefined strings
        self.buf.write(format!("%GATES%\n").as_bytes())?;

        // TODO Allow to define based on qreg names
        self.buf.write(format!("barrier q;\n").as_bytes())?;

        self.buf.write(format!("%GATES_INV%\n").as_bytes())?;

        // TODO Allow to define based on qreg names
        self.buf.write(format!("barrier q;\n").as_bytes())?;

        self.buf.write(format!("measure q -> c;\n").as_bytes())?;
    }

    #[throws(Self::Error)]
    fn visit_include(&mut self, file: &Span<Symbol>) {
        let name = &*file.inner.as_str();
        self.buf.write(format!("include \"{name}\";\n").as_bytes())?;
    }

    // TODO number is ignored
    #[throws(Self::Error)]
    fn visit_qreg(&mut self, reg: &Span<Reg>) {
        let reg = &*reg.inner;
        let name = reg.name.as_str();
        // TODO can fail
        let _i = reg.index.unwrap_or(0);

        self.buf.write(format!("qreg {name}[{}];\n", self.width).as_bytes())?;
    }

    // TODO number is ignored
    #[throws(Self::Error)]
    fn visit_creg(&mut self, reg: &Span<Reg>) {
        let reg = &*reg.inner;
        let name = reg.name.as_str();
        let _i = reg.index.unwrap_or(0);

        self.buf.write(format!("creg {name}[{}];\n", self.width).as_bytes())?;
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
    fn visit_measure(&mut self, from: &Span<Reg>, to: &Span<Reg>) {
        let from_name = &*from.inner.name.as_str();
        let to_name = &*to.inner.name.as_str();

        // TODO support index
        self.buf.write(format!("measure {from_name} -> {to_name};\n").as_bytes())?;
    }

    // TODO define all other types
}
