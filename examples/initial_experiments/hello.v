/*
 * File:    TODO.sv
 * Brief:   TODO
 *
 * Copyright (C) TODO John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

module top();
    reg a;
    child c();
    child2 c2();
endmodule

module child();
    wire a;
    grandchild gc();
endmodule

module child2();
    wire b;
endmodule

module grandchild();
    wire c;
endmodule
