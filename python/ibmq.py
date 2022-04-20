import numpy as np
from qiskit import *
from qiskit.visualization import plot_state_city
from qiskit.visualization import plot_histogram


# TODO change to run it on a real hardware
class Ibmq:
    def __init__(self):
        self.backend = None

    def auth(self):
        self.backend = Aer.get_backend('aer_simulator')
        # self.backend = Aer.get_backend('statevector_simulator')
        print('HERE')

    def run(self: 'Qiskit', circuit: str) -> str:
        print("Running")
        # print('Enabling account')
        # # IBMQ.enable_account("9ee04b444ed1c767fcd01b66027a391d8df5938df51dd27e6eaaed0a45f5da67c19dcfb2f2f46dcff893c3a54d054b4b392e1a54618d8cfea9d70d9f3378ea51")
        # # print(IBMQ.qc_providers())

        qc = QuantumCircuit.from_qasm_str(circuit)
        print(qc)

        job = self.backend.run(qc, shots=1024, memory=True)

        output = job.result().get_counts(0)

        return output
