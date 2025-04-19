import solara
from src.model import (
    FakeNewsModel,
)
from src.distributions import State
from mesa.visualization import (
    Slider,
    SolaraViz,
    make_plot_component,
    make_space_component,
)


def agent_portrayal(agent):
    node_color_dict = {
        State.INFECTED: "tab:red",
        State.SUSCEPTIBLE: "tab:green",
        State.RECOVERED: "tab:gray",
        State.EXPOSED: "tab:blue",
        State.DOUBTFUL: "tab:orange"
    }
    return {"color": node_color_dict[agent.state], "size": 10}


model_params = {
    "seed": {
        "type": "InputText",
        "value": 42,
        "label": "Random Seed",
    },
    "num_nodes": Slider(
        label="Number of agents",
        value=100,
        min=10,
        max=1000,
        step=1,
    )
}


def post_process_lineplot(ax):
    ax.set_ylim(ymin=0)
    ax.set_ylabel("# people")
    ax.legend(bbox_to_anchor=(1.05, 1.0), loc="upper left")


SpacePlot = make_space_component(agent_portrayal)
StatePlot = make_plot_component(
    {
        "Infected": "tab:red",
        "Susceptible": "tab:green",
        "Exposed": "tab:blue",
        "Doubtful": "tab:orange",
        "Recovered": "tab:gray",
    },
    post_process=post_process_lineplot,
)

model1 = FakeNewsModel()

page = SolaraViz(
    model1,
    components=[
        SpacePlot,
        StatePlot,
    ],
    model_params=model_params,
    name="Virus Model",
)
page