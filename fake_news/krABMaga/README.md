# Fake News Simulation

A simple implementation of the *Fake News Simulation* model, fully based on the krABMaga framework.

The simulation visualizes the spread of fake news in a networked environment using agent-based modeling. Each node in the network represents an individual, and their state changes over time according to predefined dynamics (e.g., exposure, infection, recovery). The framework simulates how information propagates and how individuals react — whether they believe, doubt, or recover from misinformation.

Throughout the simulation, detailed metrics are collected at each time step — specifically, the number of nodes in each of the following states:
- **Susceptible**
- **Exposed**
- **Infected**
- **Doubtful**
- **Recovered**

These statistics are used to generate dynamic plots that track the evolution of the system over time.

The visualization (available both natively and in WebAssembly) enables real-time observation of the network's behavior. Each node is represented with a colored emoji indicating its status:
- ⚪ **White Circle** – Susceptible  
- 🟢 **Green Circle** – Exposed  
- 🔴 **Red Circle** – Infected  
- ⚫ **Black Circle** – Doubtful  
- 🔵 **Blue Circle** – Recovered  

> ⚠️ Due to current limitations in the krABMaga visualization framework, **edges between nodes are static in the visualization** and do not reflect any dynamic changes that may occur during the simulation.

# How to run
- To run only the simulation, run `cargo run --release`.
- To run the native visualization, run `cargo make run --profile release`.
- To serve the web visualization locally, run `cargo make serve --profile release`.
