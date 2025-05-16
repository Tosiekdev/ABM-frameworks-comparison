# Game Of Life

A simple implementation of the Conway’s Game of Life model, fully based on the krABMaga framework.

Agents in this simulation are embedded in a `Field2D`, which by default behaves as a continuous two‑dimensional space. By passing the parameter `discretization=1` at initialization, the field is treated as a discrete grid (classic Game of Life).

When the visualization framework is enabled (either natively or compiled to WebAssembly), you can watch the evolution in real time. Alive cells are shown in black, dead cells in white, and you can easily spot gliders, blinkers, and other emergent structures as they form and interact.

# How to run
- To run only the simulation, run `cargo run --release`.
- To run the native visualization, run `cargo make run --profile release`.
- To serve the web visualization locally, run `cargo make serve --profile release`.
