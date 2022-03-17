import matplotlib.pyplot as plt
import numpy as np

from qiskit import *
from qiskit.visualization import plot_state_city
from qiskit.visualization import plot_histogram

# Run the quantum circuit on a statevector simulator backend
backend = Aer.get_backend('statevector_simulator')

def main():
    print('Enabling account')
    # # IBMQ.enable_account("9ee04b444ed1c767fcd01b66027a391d8df5938df51dd27e6eaaed0a45f5da67c19dcfb2f2f46dcff893c3a54d054b4b392e1a54618d8cfea9d70d9f3378ea51")
    # # print(IBMQ.providers())
    
    circ = QuantumCircuit(3)
    circ.h(0)
    circ.cx(0, 1)
    circ.cx(0, 2)
    print(circ)

    job = backend.run(circ)
    result = job.result()
    # print(job.result())

    outputstate = result.get_statevector(circ, decimals=3)
    print(outputstate)

    counts = result.get_counts(circ)
    print(counts)

    plot_histogram(counts)

    # plot_state_city(outputstate)

    print("Done")


if __name__ == "__main__":
    main()
