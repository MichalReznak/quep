import sys

sys.path.append('./python/.venv/lib/site-packages')
sys.path.append('./python/.venv')

from qiskit import QuantumCircuit, execute, Aer
import qiskit.providers.aer.noise as noise
import numpy as np
from qiskit import *
from qiskit.visualization import plot_state_city
from qiskit.visualization import plot_histogram
# import matplotlib.pyplot as plt

# Run the quantum circuit on a statevector simulator backend

ACCOUNT_ID = "9ee04b444ed1c767fcd01b66027a391d8df5938df51dd27e6eaaed0a45f5da67c19dcfb2f2f46dcff893c3a54d054b4b392e1a54618d8cfea9d70d9f3378ea51"

# Error probabilities
prob_1 = 0.1  # 1-qubit gate, should be around 0.01
prob_2 = 0.01   # 2-qubit gate

# Depolarizing quantum errors
error_1 = noise.depolarizing_error(prob_1, 1)
error_2 = noise.depolarizing_error(prob_2, 2)

# Add errors to noise model
noise_model = noise.NoiseModel()
noise_model.add_all_qubit_quantum_error(error_1, ['u1', 'u2', 'u3'])
noise_model.add_all_qubit_quantum_error(error_2, ['cx'])

# Get basis gates from noise model
basis_gates = noise_model.basis_gates


class Qiskit:
    def __init__(self):
        self.backend = None

    def auth(self):
        self.backend = Aer.get_backend('aer_simulator')

    def run(self: 'Qiskit', circuit: str) -> str:
        qc = QuantumCircuit.from_qasm_str(circuit)
        print(qc)

        # Perform a noise simulation
        result = execute(
            qc,
            self.backend,
            basis_gates=basis_gates,
            noise_model=noise_model
        ).result()
        counts = result.get_counts(0)

        return counts
