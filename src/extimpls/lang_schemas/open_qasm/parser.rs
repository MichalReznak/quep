use fehler::throws;
use openqasm::{Expr, ProgramVisitor, Reg, Span, Stmt, Symbol};

use crate::ext::types::lang_schema::{LangGate, LangGateType};

/// Parses gates to some size
#[derive(Default)]
pub struct ProgramParser {
    pub gates: Vec<LangGate>,
    pub qreg: i32,
    pub creg: i32,
}

impl ProgramParser {
    pub fn new() -> Self {
        Self {
            gates: vec![],
            qreg: 0,
            creg: 0,
        }
    }
}

impl ProgramVisitor for ProgramParser {
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
    fn visit_creg(&mut self, reg: &Span<Reg>) {
        if reg.name.as_str() == "c" {
            self.creg = reg.index.unwrap_or(0) as i32;
        }
    }

    #[throws(Self::Error)]
    fn visit_qreg(&mut self, reg: &Span<Reg>) {
        if reg.name.as_str() == "q" {
            self.qreg = reg.index.unwrap_or(0) as i32;
        }
    }

    #[throws(Self::Error)]
    fn visit_barrier(&mut self, regs: &[Span<Reg>]) {
        let regs: Vec<_> = (*regs).iter().map(|e| &**e).collect();

        let i = if let Some(i) = regs[0].index {
            i as i32
        }
        else {
            -1
        };
        self.gates.push(LangGate::builder().t(LangGateType::Barrier).i(i).build());
    }

    #[throws(Self::Error)]
    fn visit_gate(&mut self, name: &Span<Symbol>, _params: &[Span<Expr>], args: &[Span<Reg>]) {
        let name = (**name).to_string();
        let args: Vec<_> = (*args).iter().map(|e| &**e).collect();

        use LangGateType::*;
        let t = match name.as_str() {
            "id" => Id,
            "x" => X,
            "y" => Y,
            "z" => Z,
            "h" => H,
            "s" => S,
            "sdg" => Sdg,
            "cx" => Cx,
            "cz" => Cz,
            "swap" => Swap,

            _ => panic!("Invalid gate: {}", name.as_str()),
        };

        let gate = match t {
            Cx | Cz | Swap => LangGate::builder()
                .t(t)
                .i(args[0].index.unwrap() as i32)
                .other(args[1].index.unwrap() as i32)
                .build(),
            _ => LangGate::builder().t(t).i(args[0].index.unwrap() as i32).build(),
        };

        self.gates.push(gate);
    }
}
