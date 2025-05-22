import time
from random import Random
import numpy as np
from mesa import Model
from mesa.discrete_space import OrthogonalMooreGrid
from .agent import AgentLife


class ConwaysGameOfLife(Model):
    """Represents the 2-dimensional array of cells in Conway's Game of Life."""

    def __init__(self, scenario, steps, width, height, seed=42):
        start_time = time.time()
        """Create a new playing area based on a scenario file."""
        super().__init__(seed=seed)
        lines = []
        with open(scenario, 'r') as f:
            line = f.readline()
            while line:
                lines.append(list(line))
                line = f.readline()

        scenario = np.array(lines)

        self.height = height
        self.width = width
        random = Random(seed)
        self.grid = OrthogonalMooreGrid((self.width, self.height), capacity=1, torus=True, random=random)

        for cell in self.grid.all_cells:
            AgentLife(
                self,
                cell = cell,
                init_state=AgentLife.ALIVE
                if scenario[cell.coordinate[0]][cell.coordinate[1]] == "1"
                else AgentLife.DEAD
            )

        self.running = True
        for step in range(steps):
            self.step()

        end_time = time.time()
        print(f'total time: {end_time - start_time}')
        self.print_state()

    def step(self):
        """Perform the model step in two stages:
        - First, all cells assume their next state (whether they will be dead or alive)
        - Then, all cells change state to their next state.
        """
        self.agents.do("determine_state")
        self.agents.do("assume_state")

    def print_state(self):
        # make np array in size of grid
        states = np.zeros((self.height, self.width))
        agents = [a for a in self.agents if isinstance(a, AgentLife)]
        for agent in agents:
            if agent.is_alive:
                states[agent.cell.coordinate[0]][agent.cell.coordinate[1]] = 1

        print(states)
