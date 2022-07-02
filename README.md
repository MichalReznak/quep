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
* Send arguments to custom python extimpls

* Circuit generator should return only intermediate circuit format and then it should be serialized by lang_schema.

* Serialize to python, allow to define python extensions 
    X Can define custom requirements.txt
    X Extended with Python QcProvider
    Add option to check for constraints
    Send arguments
    Test it
    Rename Python to external (or something like that and allow it to be dynlib)
    Add other extensions to this system

* Allow to ext define custom arguments? (Nah, just write it in future work)

* Allow to define custom layer size? (Nah, just write it in future work)

* Split results from correct shots if mirror is not used
    * Do not compare only the most common number when no mirror is turn on

X Move extimpls to separate folder

* NO: Extend with toffoli gate? (This is complex gate on a circuit)

* NO: Add option to increase mirror/cycle circuit size only by one with inverted result?
  (Only good for layers of size 1, but then it's just a plain Volume benchmark that is already implemented)

* NO: Add C API? (Is it needed? I don't think so)

* NO: Extend supported gates with custom defined and their inverse
  (Will not be base gate anymore, no point of doing that)

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
