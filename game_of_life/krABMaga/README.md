# Game Of Life

A simple implementation of the Conway’s Game of Life model, fully based on the krABMaga framework.

Agents in this simulation are embedded in a `DenseGrid2D`, which is a discrete two-dimensional space. Unlike `Field2D`, `DenseGrid2D` does not provide built-in support for getting neighboring agents in the surrounding area. Therefore, a custom method `get_toroidal_neighbors` has been implemented to handle neighbor retrieval with toroidal (wrap-around) boundary conditions.

When the visualization framework is enabled (either natively or compiled to WebAssembly), you can watch the evolution in real time. Alive cells are shown in black, dead cells in white, and you can easily spot gliders, blinkers, and other emergent structures as they form and interact. Additionally, the visualization tracks and prints the elapsed time from step 0 to the final step of the simulation in the terminal.

# How to run
- To run only the simulation, run `cargo run --release <width,height> <step> <initial_state_path> [--no-terminal]`.
  
  where:

  * `<width,height>` — specifies the size of the simulation grid (e.g., `100,100`)
  * `<step>` — the number of simulation steps to run
  * `<initial_state_path>` — path to the initial state file
  * `[--no-terminal]` — optional flag to disable simulation terminal

> **Notice:** To use the visualization command below, you need to have [`cargo-make`](https://github.com/sagiegurari/cargo-make) installed.

- To run the native visualization, run `cargo make run --profile release <width,height> <step> <initial_state_path>`.
