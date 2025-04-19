import networkx as nx
from mesa import Model, DataCollector
from mesa.discrete_space import Network
from .distributions import State
from .agent import FakeNewsAgent


def number_state(model, state):
    return sum(1 for a in model.grid.all_cells.agents if a.state is state)


class FakeNewsModel(Model):
    """A virus model with some number of agents."""

    def __init__(
            self,
            num_nodes=100,
            avg_node_degree=3,
            seed=None,
    ):
        super().__init__(seed=seed)
        prob = avg_node_degree / num_nodes
        graph = nx.erdos_renyi_graph(n=num_nodes, p=prob)
        self.grid = Network(graph, capacity=1, random=self.random)

        self.datacollector = DataCollector(
            model_reporters={
                "Susceptible": lambda m: number_state(m, State.SUSCEPTIBLE),
                "Exposed": lambda m: number_state(m, State.EXPOSED),
                "Infected": lambda m: number_state(m, State.INFECTED),
                "Doubtful": lambda m: number_state(m, State.DOUBTFUL),
                "Recovered": lambda m: number_state(m, State.RECOVERED),
            },
            agent_reporters={"State": "state"}
        )

        FakeNewsAgent.create_agents(
            self,
            num_nodes,
            State.SUSCEPTIBLE,
            list(self.grid.all_cells),
        )

        self.running = True
        self.datacollector.collect(self)

    def step(self):
        self.agents.shuffle_do("step")
        self.agents.shuffle_do("advance")
        self.agents.shuffle_do("rewire")
        self.datacollector.collect(self)
