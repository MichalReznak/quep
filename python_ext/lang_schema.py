from enum import IntEnum, Enum
from typing import TypedDict, Optional


class LangGateType(str, Enum):
    X = 'X'


class LangGate(TypedDict):
    t: LangGateType
    i: int
    other: Optional[int]


class CircuitSchemaType(str, Enum):
    OpenQasm = 'OpenQasm'
    Qiskit = 'Qiskit'


class GenCircuit(TypedDict):
    circuit: str
    t: CircuitSchemaType


class LangCircuit:
    width: int
    gates: list[LangGate]


class LangSchema:
    def parse_file(path: str) -> [LangGate]:
        return [{'t': LangGateType.X, 'i': 0}]

    def as_string(self, circ: LangCircuit) -> GenCircuit:
        return {'t': CircuitSchemaType.OpenQasm, 'circuit': '''
            OPENQASM 2.0;
            include "qelib1.inc";
            
            qreg q[1];
            creg c[1];
            
            x q[0];
            x q[0];
            x q[0];
            x q[0];
            
            measure q -> c;
        '''}
