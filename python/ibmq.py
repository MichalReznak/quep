from qiskit import *

ACCOUNT_ID = "9ee04b444ed1c767fcd01b66027a391d8df5938df51dd27e6eaaed0a45f5da67c19dcfb2f2f46dcff893c3a54d054b4b392e1a54618d8cfea9d70d9f3378ea51"


# TODO change to run it on a real hardware
class Ibmq:
    def __init__(self):
        self.backend = None
        self.circuit = None

    def auth(self):
        # IBMQ.enable_account()
        # print(IBMQ.qc_providers())
        self.backend = Aer.get_backend('aer_simulator')

    def set_circuit(self: 'Ibmq', circuit: str, log: bool):
        self.circuit = QuantumCircuit.from_qasm_str(circuit)
        if log:
            print(self.circuit)

    def run(self: 'Ibmq') -> str:
        job = self.backend.run(self.circuit, shots=1024, memory=True)
        return job.result().get_counts(0)
