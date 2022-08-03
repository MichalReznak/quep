use crate::ext::types::lang_schema::LangGateType::{self, *};

pub const PAULI_GATES: [LangGateType; 4] = [Id, X, Y, Z];
pub const CLIFFORD_GATES: [LangGateType; 6] = [H, S, Id, X, Y, Z];
pub const CLIFFORD_GATES_INV: [LangGateType; 6] = [H, Sdg, Id, X, Y, Z];
pub const CLIFFORD_GATES_2: [LangGateType; 3] = [Cx, Cz, Swap]; // TODO could be with Sx, Cy
