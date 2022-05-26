use std::collections::{HashMap, HashSet};

use collection_literals::collection;
use fehler::throws;
use openqasm::{Expr, ProgramVisitor, Reg, Span, Stmt, Symbol};

/// Parses gates to some size
pub struct ProgramParser {
    depth: i32,
    pub counts: HashMap<i32, i32>,
    pub included_gates: HashSet<i32>,
    pub current_gate_i: i32,
}

impl ProgramParser {
    pub fn new(depth: i32) -> Self {
        Self {
            depth,
            counts: HashMap::new(),
            included_gates: HashSet::new(),
            current_gate_i: 0,
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
    fn visit_gate(&mut self, _name: &Span<Symbol>, _params: &[Span<Expr>], args: &[Span<Reg>]) {
        self.current_gate_i += 1;
        let args: Vec<_> = (*args).iter().map(|e| &**e).collect();

        // newly inserted gates
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

        let all = inserts.iter().all(|(k, _v)| {
            let counts = if let Some(a) = self.counts.get(k) {
                *a
            }
            else {
                0
            };
            let inserts_count = if let Some(a) = inserts.get(k) { *a } else { 0 };
            counts + inserts_count <= self.depth
        });

        for (key, val) in inserts {
            if let Some(old_val) = self.counts.get(&key) {
                self.counts.insert(key, old_val + val);
            }
            else {
                self.counts.insert(key, val);
            }
        }

        if all {
            self.included_gates.insert(self.current_gate_i);
        }
    }
}
