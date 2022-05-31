from pprint import pprint
from qiskit import *


class Simple:
    def __init__(self):
        self.backend = None
        self.circuit = None
        self.circuits = []

    def auth(self):
        # self.backend = Aer.get_backend('statevector_simulator')
        self.backend = Aer.get_backend('aer_simulator')

    def set_circuit(self: 'Simple', circuit: str, log: bool):
        self.circuit = QuantumCircuit.from_qasm_str(circuit)

        if log:
            print(self.circuit)

    def append_circuit(self: 'Simple', circuit: str, log: bool):
        qasm_circuit = QuantumCircuit.from_qasm_str(circuit)
        self.circuits.append(qasm_circuit)

        if log:
            print(qasm_circuit)

    def run(self: 'Simple') -> str:
        job = execute(self.circuit, self.backend, shots=1024, memory=True)
        return job.result().get_counts(0)

    def run_all(self: 'Simple') -> str:
        job = execute(self.circuits, self.backend, shots=1024, memory=True)
        return job.result().get_counts()
