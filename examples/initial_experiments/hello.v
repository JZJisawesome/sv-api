/*
 * File:    TODO.sv
 * Brief:   TODO
 *
 * Copyright (C) 2023 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

module top();
    reg clk;
    reg a;
    child c();
    child2 c2(.clk(clk));

    initial begin
        $display("Hello Rust from SystemVerilog!");
        #1000;
        $display("Bye Rust from SystemVerilog!");
        $finish;
    end

    initial begin
        clk = 1'b0;
        forever #5 clk = ~clk;
    end

endmodule

module child();
    wire a;
    real test = 1.0;
    grandchild gc();
endmodule

module child2(
    input wire clk
);
    reg b;
    reg [3:0] counter;
    always @(posedge clk) begin
        b <= 1'b1;
        counter <= counter + 1;
    end
endmodule

module grandchild();
    wire a;
    assign a = 1'b1;
endmodule
