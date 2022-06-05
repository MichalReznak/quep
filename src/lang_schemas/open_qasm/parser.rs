use std::collections::{HashMap, HashSet};

use collection_literals::collection;
use fehler::throws;
use openqasm::{Decl, Expr, Program, ProgramVisitor, Reg, Span, Stmt, Symbol};
use crate::ext::types::lang_schema::{LangGate, LangGateType};

/// Parses gates to some size
pub struct ProgramParser {
    pub gates: Vec<LangGate>,
}

impl ProgramParser {
    pub fn new() -> Self {
        Self {
            gates: vec![],
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
    fn visit_barrier(&mut self, regs: &[Span<Reg>]) {
        // TODO
    }

    #[throws(Self::Error)]
    fn visit_gate(&mut self, name: &Span<Symbol>, _params: &[Span<Expr>], args: &[Span<Reg>]) {
        let name = (&**name).to_string();
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

            _ => panic!("Invalid gate"),
        };

        let gate = match t {
            Cx|Cz|Swap => {
                LangGate::builder().t(t).i(args[0].index.unwrap() as i32).other(args[1].index.unwrap() as i32).build()
            },
            _ => {
                LangGate::builder().t(t).i(args[0].index.unwrap() as i32).build()
            }
        };

        self.gates.push(gate);
    }
}