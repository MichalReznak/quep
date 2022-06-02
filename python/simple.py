from qiskit import *


class Simple:
    def __init__(self):
        self.backend = None
        self.circuits = []

    def auth(self):
        # self.backend = Aer.get_backend('statevector_simulator')
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
        job = execute(self.circuits, self.backend, shots=1024, memory=True)
        print(job.result().get_counts())
        return job.result().get_counts()
