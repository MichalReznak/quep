OPENQASM 2.0;
include "qelib1.inc";

qreg q[2];
creg c[1];

reset q[0];
reset q[1];

h q[0];
x q[1];
h q[1];

// Oracle
barrier q;
x q[0];

cx q[0], q[1];

x q[0];
barrier q;

h q[0];

measure q[0] -> c[0];
