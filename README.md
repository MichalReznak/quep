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
* TODO extend supported gates with custom defined and their inverse
* Split results from correct shots if mirror is not used
  * TODO do not compare only the most common number when no mirror is turn on
* Extend with toffoli gate?

#### Changes after image inserted:
X Added mirror flag
X Lot of other stuff I don't remember anymore
X Add arg to say whether circuits should be mirrored or computed on a simulator
X implement cycle benchmarking (inverse gate is inserted directly behind the correct gate, otherwise it is the same as mirror)
X allow to define initial state (does not need to be all zeros)
X Add dependencies to maturin
X include python scripts as strings
X rename mirror and rand mirror to structured and random generators
X Add constraints
X Add a workaround for odd circuit for mirror circuits
X add from_size and from_size_2 options
