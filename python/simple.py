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

    def append_circuit(self: 'Simple', circuit: str, t: str, log: bool):
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

    def run_all(self: 'Simple') -> str:
        start = time.time()
        job = execute(self.circuits, self.backend, shots=1024, memory=True)
        end = time.time()

        self.meta_info = {
            'time': end - start
        }

        print(job.result().get_counts())
        return job.result().get_counts()
