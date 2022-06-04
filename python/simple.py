import time
from qiskit import *


class Simple:
    meta_info: dict[str, any] = None
    backend: any = None
    circuits: [QuantumCircuit] = []

    def get_meta_info(self):
        return self.meta_info

    def auth(self):
        self.backend = Aer.get_backend('aer_simulator')

    def clear_circuits(self: 'Simple'):
        self.circuits = []

    def set_circuit(self: 'Simple', circuit: str, log: bool):
        qasm_circuit = QuantumCircuit.from_qasm_str(circuit)
        self.circuits = [qasm_circuit]

        if log:
            print(qasm_circuit)

    def append_circuit(self: 'Simple', circuit: str, log: bool):
        qasm_circuit = QuantumCircuit.from_qasm_str(circuit)
        self.circuits.append(qasm_circuit)

        if log:
            print(qasm_circuit)

    def run_all(self: 'Simple') -> str:
        start = time.time()
        job = execute(self.circuits, self.backend, shots=1024, memory=True)
        end = time.time()

        self.meta_info = {
            'time': end - start
        }

        print(job.result().get_counts())
        return job.result().get_counts()
