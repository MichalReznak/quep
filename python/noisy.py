from qiskit import *
from qiskit.providers.aer import AerSimulator
from qiskit.providers.aer.noise import NoiseModel, pauli_error

# TODO
# p_reset = 0.003
# p_meas = 0.001
# p_gate1 = 0.005

p_reset = 0.5
p_meas = 0.05
p_gate1 = 0.05

# QuantumError objects
error_reset = pauli_error([('X', p_reset), ('I', 1 - p_reset)])
error_meas = pauli_error([('X', p_meas), ('I', 1 - p_meas)])
error_gate1 = pauli_error([('X', p_gate1), ('I', 1 - p_gate1)])
error_gate2 = error_gate1.tensor(error_gate1)

# Add errors to noise model
noise_bit_flip = NoiseModel()
noise_bit_flip.add_all_qubit_quantum_error(error_reset, "reset")
noise_bit_flip.add_all_qubit_quantum_error(error_meas, "measure")
noise_bit_flip.add_all_qubit_quantum_error(error_gate1, ["u1", "u2", "u3"])
noise_bit_flip.add_all_qubit_quantum_error(error_gate2, ["cx"])


class Noisy:
    def auth(self):
        pass

    def run(self: 'Qiskit', circuit: str) -> str:
        qc = QuantumCircuit.from_qasm_str(circuit)
        print(qc)

        sim_noise = AerSimulator(noise_model=noise_bit_flip)

        # Transpile circuit for noisy basis gates
        circ_tnoise = transpile(qc, sim_noise)

        # Run and get counts
        result_bit_flip = sim_noise.run(circ_tnoise).result()
        counts_bit_flip = result_bit_flip.get_counts(0)


        # Perform a noise simulation
        # result = execute(
        #     qc,
        #     self.backend,
        #     basis_gates=basis_gates,
        #     noise_model=noise_model
        # ).result()
        # counts = result.get_counts(0)

        return counts_bit_flip
