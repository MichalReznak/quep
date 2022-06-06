from qiskit import *

circ = QuantumCircuit(3, 3)
circ.h(0)
circ.cx(0, 1)
circ.cx(0, 2)
circ.measure(0, 0)
circ.measure(0, 1)
circ.measure(0, 2)
