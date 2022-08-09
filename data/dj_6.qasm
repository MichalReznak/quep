OPENQASM 2.0;
include "qelib1.inc";

qreg q[6];
creg c[5];

reset q[0];
reset q[1];
reset q[2];
reset q[3];
reset q[4];
reset q[5];

h q[0];
h q[1];
h q[2];
h q[3];
h q[4];
x q[5];
h q[5];

// Oracle
barrier q;
x q[0];
x q[1];
x q[4];

cx q[0], q[5];
cx q[1], q[5];
cx q[2], q[5];
cx q[3], q[5];
cx q[4], q[5];

x q[0];
x q[1];
x q[4];
barrier q;

h q[0];
h q[1];
h q[2];
h q[3];
h q[4];

measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
measure q[3] -> c[3];
measure q[4] -> c[4];
