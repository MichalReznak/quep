import sys

sys.path.append('./python/.venv/lib/site-packages')
sys.path.append('./python/.venv')


# import matplotlib.pyplot as plt
import numpy as np

from qiskit import *
from qiskit.visualization import plot_state_city
from qiskit.visualization import plot_histogram

# Run the quantum circuit on a statevector simulator backend

class Qiskit:
    def auth(self):
        self.backend = Aer.get_backend('aer_simulator')
        print('HERE')


    def run(self: 'Qiskit', circuit: str) -> str:
        print("Running")
        # print('Enabling account')
        # # IBMQ.enable_account("9ee04b444ed1c767fcd01b66027a391d8df5938df51dd27e6eaaed0a45f5da67c19dcfb2f2f46dcff893c3a54d054b4b392e1a54618d8cfea9d70d9f3378ea51")
        # # print(IBMQ.qc_providers())
        
        qc = QuantumCircuit.from_qasm_str(circuit)
        print(qc)

        job = self.backend.run(qc, shots=1, memory=True)
        output = job.result().get_memory()[0]
        print(output)

        return output
