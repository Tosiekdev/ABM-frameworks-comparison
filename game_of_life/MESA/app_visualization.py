from src.model import ConwaysGameOfLife
from mesa.visualization import (
    SolaraViz,
    make_space_component,
)

def agent_portrayal(agent):
    return {
        "color": "white" if agent.state == 0 else "black",
        "marker": "s",
        "size": 25,
    }

def post_process(ax):
    ax.set_aspect("equal")
    ax.set_xticks([])
    ax.set_yticks([])

# Create model instance with fixed inputs
model = ConwaysGameOfLife(
    scenario="./game_of_life/test_scenarios/10.txt",
    steps=10,
    seed=42,
    width=10,
    height=10,
)

# Create the space component for visualization
SpaceGraph = make_space_component(
    agent_portrayal, post_process=post_process, draw_grid=False
)

# Create the SolaraViz page with no dynamic model_params
page = SolaraViz(
    model,
    components=[SpaceGraph],
    model_params=None,
    name="Game of Life",
)
