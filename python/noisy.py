from qiskit import *
from qiskit.providers.aer import AerSimulator
from qiskit.providers.aer.noise import NoiseModel, pauli_error, depolarizing_error

p_reset = 0.002
p_meas = 0.001
p_gate1 = 0.003

# p_reset = 0.05
# p_meas = 0.05
# p_gate1 = 0.02


# QuantumError objects
error_reset = pauli_error([('X', p_reset), ('I', 1 - p_reset)])
error_meas = pauli_error([('X', p_meas), ('I', 1 - p_meas)])
error_gate1 = pauli_error([('X', p_gate1), ('I', 1 - p_gate1)])
error_gate2 = error_gate1.tensor(error_gate1)

error_reset_p = pauli_error([('Z', p_reset), ('I', 1 - p_reset)])
error_meas_p = pauli_error([('Z', p_meas), ('I', 1 - p_meas)])
error_gate1_p = pauli_error([('Z', p_gate1), ('I', 1 - p_gate1)])
error_gate2_p = error_gate1.tensor(error_gate1)

error_reset_p2 = error_reset.compose(error_reset_p)
error_meas_p2 = error_meas.compose(error_meas_p)
error_gate1_p2 = error_gate1.compose(error_gate1_p)
error_gate2_p2 = error_gate2.compose(error_gate2_p)

# TODO option 2
# error_reset_p2 = depolarizing_error(p_reset, 1)
# error_meas_p2 = depolarizing_error(p_meas, 1)
# error_gate1_p2 = depolarizing_error(p_gate1, 1)
# error_gate2_p2 = error_gate1_p2.tensor(error_gate1_p2)


# Add errors to noise model
noise_bit_flip = NoiseModel()
noise_bit_flip.add_all_qubit_quantum_error(error_reset_p2, "reset")
noise_bit_flip.add_all_qubit_quantum_error(error_meas_p2, "measure")
noise_bit_flip.add_all_qubit_quantum_error(error_gate1_p2, ["id", "rz", "u1", "u2", "u3"])
noise_bit_flip.add_all_qubit_quantum_error(error_gate2_p2, ["cx", "zx"])


class Noisy:
    def __init__(self):
        self.backend = None
        self.circuit = None
        self.circuits = []

    def auth(self):
        self.backend = AerSimulator(noise_model=noise_bit_flip)

    def set_circuit(self: 'Noisy', circuit: str, log: bool):
        self.circuit = QuantumCircuit.from_qasm_str(circuit)

        if log:
            print(self.circuit)

    def append_circuit(self: 'Noisy', circuit: str, log: bool):
        qasm_circuit = QuantumCircuit.from_qasm_str(circuit)
        self.circuits.append(qasm_circuit)

        if log:
            print(qasm_circuit)

    def run(self: 'Noisy') -> str:
        job = execute(self.circuit, self.backend, shots=1024, memory=True)
        return job.result().get_counts(0)

    def run_all(self: 'Noisy') -> str:
        job = execute(self.circuits, self.backend, shots=1024, memory=True)
        return job.result().get_counts()
