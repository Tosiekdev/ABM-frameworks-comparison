# Fake News Spread Simulation (Mesa + Solara)

This project simulates the spread of fake news in a population using an agent-based model built with the [Mesa](https://mesa.readthedocs.io/en/stable/) framework and [Solara](https://github.com/widgetti/solara) for visualization.

## 🧠 Overview

Agents exist in a social network where they can be in one of five states:
- **S** (Susceptible)
- **E** (Exposed)
- **I** (Infected - believing and spreading misinformation)
- **D** (Doubtful - skeptical)
- **R** (Recovered - resistant to misinformation)

They transition between states based on demographic factors and their local network environment. Agents also rewire connections to favor similar-minded individuals.

## 🛠️ Requirements
- Python 3.10+
- [Mesa](https://pypi.org/project/mesa/)
- [Solara](https://pypi.org/project/solara/)
- [networkx](https://pypi.org/project/networkx/)

Install all dependencies (ideally inside a virtual environment)

## 🚀 How to Run the Simulation

Use Solara to launch the dashboard:

```bash
solara run src/app.py
```

Make sure you run the command from the **project root directory**, and that your file structure includes:

```
MESA/
├── src/
│   ├── app.py           # main Solara UI
│   ├── agent.py         # FakeNewsAgent logic
│   ├── model.py         # FakeNewsModel logic
│   └── distributions.py # enums + distributions
└── README.md
```

Once running, the simulation UI will appear in your browser at `http://localhost:8765/`.

