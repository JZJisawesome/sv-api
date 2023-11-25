# sv-api

Idiomatic Rust abstractions and raw bindings for the SystemVerilog DPI, PLI, and VPI interfaces.

This crate is in a early development stage at the moment as I flesh out the goals of the project, but it will improve with time :)

TODO describe distinction between this and librstb (or perhaps even reuse parts of that)
- Unlike librstb we're not focused on writing tests (assuming those will be written in SystemVerilog, potentially leveraging UVM, in the simulator)
- Instead our focus is on simply providing a good abstraction to control a simulation, as well as retrieve data from it
