from qiskit import *

ACCOUNT_ID = "9ee04b444ed1c767fcd01b66027a391d8df5938df51dd27e6eaaed0a45f5da67c19dcfb2f2f46dcff893c3a54d054b4b392e1a54618d8cfea9d70d9f3378ea51"

# TODO use time_per_step() for time estimates
class Ibmq:
    def __init__(self, account_id: str):
        self.backend = None
        self.circuit = None
        self.circuits = []
        self.account_id = account_id

    def auth(self):
        IBMQ.enable_account(self.account_id)
        self.backend = IBMQ.providers()[0].get_backend('ibmq_quito')  # TODO allow to define

    def set_circuit(self: 'Ibmq', circuit: str, log: bool):
        self.circuit = QuantumCircuit.from_qasm_str(circuit)
        if log:
            print(self.circuit)

    def append_circuit(self: 'Ibmq', circuit: str, log: bool):
        qasm_circuit = QuantumCircuit.from_qasm_str(circuit)
        self.circuits.append(qasm_circuit)

        if log:
            print(qasm_circuit)

    def run(self: 'Ibmq') -> str:
        print("Waiting for execution...")
        job = execute(self.circuit, self.backend, shots=1024, memory=True)
        return job.result().get_counts(0)

    def run_all(self: 'Ibmq') -> str:
        print("Waiting for execution...")
        job = execute(self.circuits, self.backend, shots=1024, memory=True)
        return job.result().get_counts()
