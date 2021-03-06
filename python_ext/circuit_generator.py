from enum import Enum
from typing import TypedDict, Optional


class LangGateType(str, Enum):
    X = 'X'


class LangGate(TypedDict):
    t: LangGateType
    i: int
    other: Optional[int]


class LangCircuit(TypedDict):
    width: int
    gates: list[LangGate]


class CircuitGenerator:
    def __init__(self, args):
        pass

    def check_constraints(self, config) -> dict[str, any]:
        return {'correct': False, 'reason': 'Some wild reason'}

    def generate(self, i: int, j: int, it: int) -> LangCircuit:
        return {
            'width': 4,
            'gates': [{'t': LangGateType.X, 'i': 0, 'other': 42}]
        }
