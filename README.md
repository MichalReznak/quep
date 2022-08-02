# quep
Quantum performance benchmark tool

## Build
* Need nightly version of the rust toolchain

## Code coverage
* https://github.com/mozilla/grcov
```bash
grcov . \
    -s . \
    --binary-path ./target/debug/ \
    -t html \
    --branch \
    --ignore-not-existing -o ./target/debug/coverage/ \
    --ignore "target/debug/*"
```


```powershell
grcov . `
    -s . `
    --binary-path ./target/debug/ `
    -t html `
    --branch `
    --ignore-not-existing -o ./target/debug/coverage/ `
    --ignore "target/debug/*"
```

##### TODOs:
* Quantum Volumes, Struct and Rand should pseudo randomize neighboring qubits [config option 'shuffle'?]

* Second layer in struct and rand algorithms should be two qubit gates only (to have some qubits coherence)

X Improve time estimates for Ibmq: time_per_step()

X Wrong index for Serial output with single orchestrator

X Skip zero values in output so fromSize is not written as 0

##### Skipped (Future work):
X NO: Allow to define custom layer size? (Nah, just write it in future work)

X NO: Extend with toffoli gate? (This is complex gate on a circuit)

X NO: Add option to increase mirror/cycle circuit size only by one with inverted result?
  (Only good for layers of size 1, but then it's just a plain Volume benchmark that is already implemented)

X NO: Add C API? (Is it needed? I don't think so)

X NO: Extend supported gates with custom defined and their inverse
  (Will not be base gate anymore, no point of doing that)

X NO: From Lang Schema allow to define custom registers (Extension)

X NO: Allow to ext define custom arguments? (Nah, just write it in future work)


#### Changes after image inserted:
X Quantum volume should can use only two qubit gates
(Number is too low, so for the start single qubit gates could be better)
https://medium.com/qiskit/what-is-quantum-volume-anyway-a4dff801c36f

X Add arg to say whether circuits should be mirrored or computed on a simulator
X implement cycle benchmarking (inverse gate is inserted directly behind the correct gate, otherwise it is the same as mirror)
X allow to define initial state (does not need to be all zeros)
X Add dependencies to maturin
X include python scripts as strings
X rename mirror and rand mirror to structured and random generators
X Add constraints
X Add a workaround for odd circuit for mirror circuits
X add from_size and from_size_2 options
X Send arguments to custom python extimpls
X Move extimpls to separate folder
X Circuit generator should return only intermediate circuit format and then it should be serialized by lang_schema.
X Serialize to python, allow to define python extensions
    X Can define custom requirements.txt
    X Extended with Python QcProvider
    X Add option to check for constraints
    X Send arguments
    X Test it
