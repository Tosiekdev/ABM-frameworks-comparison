from mesa import Model, DataCollector
from mesa.experimental.continuous_space import ContinuousSpace
from .distributions import State
from .agent import FakeNewsAgent
from random import random
import numpy as np


def number_state(model, state):
    return sum(1 for a in model.agents if a.state == state)


class FakeNewsModel(Model):
    """A virus model with some number of agents."""

    def __init__(
            self,
            size=100,
            n_agents=1000,
            seed=None
    ):
        super().__init__(seed=seed)
        self.grid = ContinuousSpace([[0, size],[0, size]])

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

        for _ in range(n_agents):
            x = random() * size
            y = random() * size
            FakeNewsAgent(self.grid, self, State.SUSCEPTIBLE, np.array([x, y]))
        self.running = True
        self.datacollector.collect(self)

    def step(self):
        self.agents.shuffle_do("step")
        self.agents.shuffle_do("advance")
        self.agents.shuffle_do("move")
        self.datacollector.collect(self)
