from qiskit import *
from qiskit.visualization import plot_histogram
import matplotlib


class Qiskit:
    def __init__(self):
        self.backend = None

    def auth(self):
        # self.backend = Aer.get_backend('statevector_simulator')
        self.backend = Aer.get_backend('aer_simulator')

    def run(self: 'Qiskit', circuit: str) -> str:
        qc = QuantumCircuit.from_qasm_str(circuit)
        print(qc)  # TODO only on debug

        job = self.backend.run(qc, shots=1024, memory=True)
        output = job.result().get_counts(0)

        # TODO plot
        # plot_histogram(output)
        # matplotlib.pyplot.show(block=True)  # To show the graph

        return output
