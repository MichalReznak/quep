OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

reset q[0];
x q[0];

measure q -> c;
