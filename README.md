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

* Add arg to say whether circuits should be mirrored or computed on a simulator
* Split results from correct shots if mirror is not used
* TODO Allow to change mirror to output all ones
* TODO do nto compare only the most common number when no mirror is turn on
* TODO Add a workaround for odd circuit for mirror circuits
* TODO implement cycle benchmarking (inverse gate is inserted directly behind the correct gate, otherwise it is the same as mirror)
* TODO allow to define initial state (does not need to be all zeros)
X TODO include python scripts as strings
* TODO Add dependencies to maturin

#### Changes after image inserted:
* Added mirror flag
