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
* Skip zero values in output so fromSize is not written as 0

* Quantum volume should use only two qubit gates
  (or allow to choose? Number is too low, so for teh start single qubit gates could be better)
  https://medium.com/qiskit/what-is-quantum-volume-anyway-a4dff801c36f

* Improve time estimates: time_per_step()


##### Skipped (Future work):
* NO: Allow to define custom layer size? (Nah, just write it in future work)

* NO: Extend with toffoli gate? (This is complex gate on a circuit)

* NO: Add option to increase mirror/cycle circuit size only by one with inverted result?
  (Only good for layers of size 1, but then it's just a plain Volume benchmark that is already implemented)

* NO: Add C API? (Is it needed? I don't think so)

* NO: Extend supported gates with custom defined and their inverse
  (Will not be base gate anymore, no point of doing that)

* NO: From Lang Schema allow to define custom registers (Extension)

* NO: Allow to ext define custom arguments? (Nah, just write it in future work)


#### Changes after image inserted:
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
