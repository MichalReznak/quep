from qiskit import *


# TODO use time_per_step() for time estimates
class Ibmq:
    def __init__(self, account_id: str):
        self.backend = None
        self.circuits = []
        self.account_id = account_id

    def auth(self):
        IBMQ.enable_account(self.account_id)
        self.backend = IBMQ.providers()[0].get_backend('ibmq_quito')  # TODO allow to define

    def clear_circuits(self: 'Ibmq'):
        self.circuits = []

    def append_circuit(self: 'Ibmq', circuit: str, log: bool):
        qasm_circuit = QuantumCircuit.from_qasm_str(circuit)
        self.circuits.append(qasm_circuit)

        if log:
            print(qasm_circuit)

    def run_all(self: 'Ibmq') -> str:
        print("Waiting for execution...")
        job = execute(self.circuits, self.backend, shots=1024, memory=True)
        return job.result().get_counts()
