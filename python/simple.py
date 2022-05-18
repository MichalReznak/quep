from qiskit import *

# TODO plot
# from qiskit.visualization import plot_histogram
# import matplotlib
# plot_histogram(output)
# matplotlib.pyplot.show(block=True)  # To show the graph


class Simple:
    def __init__(self):
        self.backend = None
        self.circuit = None

    def auth(self):
        # self.backend = Aer.get_backend('statevector_simulator')
        self.backend = Aer.get_backend('aer_simulator')

    def set_circuit(self: 'Simple', circuit: str, log: bool):
        self.circuit = QuantumCircuit.from_qasm_str(circuit)

        if log:
            print(self.circuit)

    def run(self: 'Simple') -> str:
        job = self.backend.run(self.circuit, shots=1024, memory=True)
        return job.result().get_counts(0)
