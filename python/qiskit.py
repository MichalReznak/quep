import numpy as np
from qiskit import *
from qiskit.visualization import plot_state_city
from qiskit.visualization import plot_histogram


class Qiskit:
    def __init__(self):
        self.backend = None

    def auth(self):
        # self.backend = Aer.get_backend('statevector_simulator')
        self.backend = Aer.get_backend('aer_simulator')

    def run(self: 'Qiskit', circuit: str) -> str:
        qc = QuantumCircuit.from_qasm_str(circuit)
        print(qc)

        job = self.backend.run(qc, shots=1024, memory=True)
        output = job.result().get_counts(0)
        return output
