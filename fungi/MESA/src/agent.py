import numpy as np
from mesa import Agent
import random
from mesa.discrete_space.cell_agent import CellAgent

DEAD = 0
ALIVE = 1


class FungusCell(CellAgent, Agent):
    def __init__(self, model, pos, resource_amount=10):
        super().__init__(model)
        self.cell = pos
        self.resource_amount = resource_amount
        self.state = ALIVE
        self.pos = pos

    def share(self):
        if self.resource_amount <= 3:
            return 0
        elif self.resource_amount < 10:
            share_low = np.floor(self.resource_amount / 2)
            self.resource_amount -= share_low
            return share_low
        else:
            share = np.floor(self.resource_amount * 0.8)
            self.resource_amount -= share
            return share

    def share_with_neighbour(self):
        target_cells = [
            cell for cell in self.cell.neighborhood
            if any(isinstance(obj, CoreCell) and obj.resource_amount < self.resource_amount for obj in cell.agents)
        ]

        if not target_cells:
            return

        amount = self.share()
        amount = np.floor(amount / len(target_cells))

        for cell in target_cells:
            for agent in cell.agents:
                if isinstance(agent, CoreCell) and agent.resource_amount < self.resource_amount:
                    agent.resource_amount += amount

    def decide_development(self):
        pass

    def decide_exploration(self):
        pass

    def feed(self):
        if self.state == DEAD:
            return

        available = self.cell.resource
        if available > 0:
            harvested = min(10, available)
            self.cell.resource -= harvested
            self.resource_amount += harvested

        self.share_with_neighbour()

        if self.resource_amount <= 1:
            self.state = DEAD

        self.use_resource()

    def use_resource(self):
        self.resource_amount -= self.model.resource_consumption


class CoreCell(FungusCell):
    def __init__(self, model, pos, resource_amount=20):
        super().__init__(model, pos, resource_amount)
        self.direct_opposite_chance = 0.3
        self.broad_opposite_chance = 0.1

    def decide_development(self):
        if self.state == DEAD:
            return

        empty_cells = [
            cell for cell in self.cell.neighborhood
            if not any(isinstance(obj, FungusCell) for obj in cell.agents)
        ]

        if not empty_cells:
            return

        probabilities = []
        for cell in empty_cells:
            prob = 1.0
            for neighbor in cell.neighborhood:
                if any(isinstance(obj, FungusCell) for obj in neighbor.agents):
                    prob += self.direct_opposite_chance
            probabilities.append(prob)

        total = sum(probabilities)
        probabilities = [p / total for p in probabilities]

        chosen_cell = random.choices(empty_cells, weights=probabilities, k=1)[0]
        shared_resource = self.share()

        if shared_resource > 0:
            if random.random() < 0.1:
                new_explorer = ExplorationCell(self.model, chosen_cell, shared_resource)
                self.model.register_agent(new_explorer)
            else:
                new_core = CoreCell(self.model, chosen_cell, shared_resource)
                self.model.register_agent(new_core)


class ExplorationCell(FungusCell):
    #ToDo:

    def __init__(self, model, pos, resource_amount=10.0):
        super().__init__(model, pos, resource_amount)
        self.is_landlocked = False
        self.direct_opposite_chance = 0.9
        self.broad_opposite_chance = 0.1

    def decide_exploration(self):
        if self.is_landlocked:
            return
        cells = [
            cell for cell in self.cell.neighborhood
            if any(isinstance(obj, ExplorationCell) for obj in cell.agents)
        ]
        if len(cells) > 2:
            self.is_landlocked = True
            return

        empty_cells = [
            cell for cell in self.cell.neighborhood
            if not any(isinstance(obj, FungusCell) for obj in cell.agents)
        ]

        if not empty_cells:
            return

        probabilities = []
        for cell in empty_cells:
            prob = 1.0
            for neighbor in cell.neighborhood:
                if any(isinstance(obj, FungusCell) for obj in neighbor.agents):
                    prob += self.direct_opposite_chance
            probabilities.append(prob)

        total = sum(probabilities)
        probabilities = [p / total for p in probabilities]

        chosen_cell = random.choices(empty_cells, weights=probabilities, k=1)[0]


        if chosen_cell.resource > 0:
            shared_resource = self.share()
            # Found resource — spawn CoreCell in that direction
            new_core = CoreCell(self.model, chosen_cell, shared_resource)
            self.model.register_agent(new_core)
            return  # Skip exploration for now (this could be adjusted)

        new_explorer = ExplorationCell(self.model, chosen_cell, 10)
        self.model.register_agent(new_explorer)

    def feed(self):
        empty_cells = [
            cell for cell in self.cell.neighborhood
            if not any(isinstance(obj, FungusCell) for obj in cell.agents)
        ]
        if len(empty_cells) >= 7:
            return

        available = self.cell.resource
        if available > 0:
            harvested = min(5, available)
            self.cell.resource -= harvested
            self.resource_amount += harvested

        self.share_with_neighbour()
