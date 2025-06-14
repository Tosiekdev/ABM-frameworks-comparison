# Fake News Spread Simulation (Mesa + Solara)

This project simulates the spread of fake news in a population using an agent-based model built with the [Mesa](https://mesa.readthedocs.io/en/stable/) framework and [Solara](https://github.com/widgetti/solara) for visualization.

## 🧠 Overview

Agents exist in a continuous space where they can be in one of five states:
- **S** (Susceptible)
- **E** (Exposed)
- **I** (Infected - believing and spreading misinformation)
- **D** (Doubtful - skeptical)
- **R** (Recovered - resistant to misinformation)

They transition between states based on demographic factors and their local network environment. Agents also move in the dircetion of the similar-minded individuals.

## 🛠️ Requirements
- Python 3.10+
- [Mesa](https://pypi.org/project/mesa/)
- [Matplotlib](https://pypi.org/project/matplotlib)

Install all dependencies (ideally inside a virtual environment)

## 🚀 How to Run the Simulation

```bash
python app.py
```

Make sure you run the command from the **project root directory**, and that your file structure includes:

```
MESA/
├── src/
│   ├── agent.py         # FakeNewsAgent logic
│   ├── model.py         # FakeNewsModel logic
│   └── distributions.py # enums + distributions
├── app.py               # application entry point
└── README.md
```

