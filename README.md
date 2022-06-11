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

# TODOs:
* TODO Add constraints

* TODO use thermal relaxation example to create a better noisy simulator
* TODO extend supported gates with custom defined and their inverse
* TODO C/Python-API to the library, C-API for extensions
* Use JobManager to Ibmq https://qiskit.org/documentation/stubs/qiskit.providers.ibmq.managed.IBMQJobManager.html#qiskit.providers.ibmq.managed.IBMQJobManager
* Generate documentation
* Add arg to say whether circuits should be mirrored or computed on a simulator
* Split results from correct shots
