OPENQASM 2.0;
include "qelib1.inc";

qreg q[3];
creg c[2];

reset q[0];
reset q[1];
reset q[2];

h q[0];
h q[1];
x q[2];
h q[2];

// Oracle
barrier q;
x q[0];
x q[1];

cx q[0], q[2];
cx q[1], q[2];

x q[0];
x q[1];
barrier q;

h q[0];
h q[1];

measure q[0] -> c[0];
measure q[1] -> c[1];
