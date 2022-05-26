// OPENQASM 2.0;
// include "qelib1.inc";

// qreg q[1];
// creg c[1];

// reset q[0];
// z q[0];
// x q[0];

// ch q[0], q[1];

cx q[0], q[1];

// x q[1];

// s q[2];

cx q[2], q[1];

// s q[2];

// s q[2];

// cx q[0], q[1];

// x q[1];

x q[2];

cx q[0], q[1];

x q[0];

s q[3];

measure q -> c;
