OPENQASM 2.0;
include "qelib1.inc";

qreg q[7];
creg c[7];

reset q[0];
reset q[1];
reset q[2];
reset q[3];
reset q[4];
reset q[5];
reset q[6];

h q[0];
h q[1];
h q[2];
h q[3];
h q[4];
h q[5];
x q[6];
h q[6];

// Oracle
barrier q;
x q[0];
x q[1];
x q[4];

cx q[0], q[6];
cx q[1], q[6];
cx q[2], q[6];
cx q[3], q[6];
cx q[4], q[6];
cx q[5], q[6];

x q[0];
x q[1];
x q[4];
barrier q;

h q[0];
h q[1];
h q[2];
h q[3];
h q[4];
h q[5];

measure q -> c;
