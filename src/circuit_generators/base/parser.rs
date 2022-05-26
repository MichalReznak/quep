use std::collections::{HashMap, HashSet};

use collection_literals::collection;
use fehler::throws;
use openqasm::{Expr, ProgramVisitor, Reg, Span, Stmt, Symbol};

/// Parses gates to some size
pub struct ProgramParser {
    depth: i32,
    width: i32,
    pub counts: HashMap<i32, i32>,
    pub included_gates: HashSet<i32>,
    pub current_gate_i: i32,
}

impl ProgramParser {
    pub fn new(depth: i32, width: i32) -> Self {
        Self {
            depth,
            width,
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
        let args: Vec<_> = (*args).iter().map(|e| &**e).collect();
        // Newly inserted gates
        let mut inserts = collection! { HashMap<i32, i32> };

        // TODO do not depend on index
        // Count number of indices
        for arg in args {
            let i = arg.index.unwrap() as i32;

            // Skip gates with index out of range
            if i >= self.width {
                self.current_gate_i += 1;
                return;
            }

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

        // Do all indices fulfill the limit?
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

        if all {
            self.included_gates.insert(self.current_gate_i);

            // Add limits for the next gate
            for (key, val) in inserts {
                if let Some(old_val) = self.counts.get(&key) {
                    self.counts.insert(key, old_val + val);
                }
                else {
                    self.counts.insert(key, val);
                }
            }
        }

        self.current_gate_i += 1;
    }
}
