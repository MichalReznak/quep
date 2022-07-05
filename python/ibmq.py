import time
from qiskit import *


class Ibmq:
    backend: any = None
    circuits: [QuantumCircuit] = []
    account_id: str = None
    machine_name: str = None
    meta_info: dict[str, any] = None

    def __init__(self, account_id: str, machine_name: str):
        self.account_id = account_id
        self.machine_name = machine_name

    def get_meta_info(self):
        return self.meta_info

    def auth(self):
        IBMQ.enable_account(self.account_id)
        self.backend = IBMQ.providers()[0].get_backend(self.machine_name)

    def clear_circuits(self: 'Ibmq'):
        self.circuits = []

    def append_circuit(self: 'Ibmq', circuit: str, t: str, log: bool):
        parsed_c = None

        if t == 'OpenQasm':
            parsed_c = QuantumCircuit.from_qasm_str(circuit)

        elif t == 'Qiskit':
            exec_res = {}
            exec(circuit, None, exec_res)
            parsed_c = exec_res["circ"]

        self.circuits.append(parsed_c)

        if log:
            print(parsed_c)

    def run_all(self: 'Ibmq') -> str:
        print("Waiting for execution...")

        start = time.time()
        job = execute(self.circuits, self.backend, shots=1024, memory=True, optimization_level=0)
        end = time.time()

        self.meta_info = {
            'time': end - start
        }

        return job.result().get_counts()
