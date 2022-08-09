OPENQASM 2.0;
include "qelib1.inc";

qreg q[4];
creg c[3];

reset q[0];
reset q[1];
reset q[2];
reset q[3];

h q[0];
h q[1];
h q[2];
x q[3];
h q[3];

// Oracle
barrier q;
x q[0];
x q[1];
x q[2];

cx q[0], q[3];
cx q[1], q[3];
cx q[2], q[3];

x q[0];
x q[1];
x q[2];
barrier q;

h q[0];
h q[1];
h q[2];

measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
