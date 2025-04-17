# Fungi Growth Simulation 
A simple implementation of the *Fungi Growth Simulation* model, fully based on the krABMaga framework. 

Throughout the simulation, detailed metrics are collected at each time step — including the number of **core** and **exploration** cells, as well as the current number of available resources. These metrics are gathered to generate dynamic plots that visualize how the system evolves over time.

The simulation with the visualization framework enabled (either natively or compiled to WebAssembly) allows viewers to observe the behavior of the fungal network in real time. Fungi agents dynamically spread across the environment in search of resources. Two types of agents — **core** and **exploration** cells — can be visually distinguished by their color: core cells are shown in **black**, while exploration cells appear in **gray**. As they grow and interact with their surroundings, these visual cues help highlight the structure and expansion of the fungal network. Resource distribution is represented through the color green, indicating availability. 

# How to run
- To run only the simulation, run `cargo run --release`.
- To run the native visualization, run `cargo make run --profile release`.
- To serve the web visualization locally, run `cargo make serve --profile release`.
  