import numpy as np
from mesa import Agent
import random

DEAD = 0
ALIVE = 1


class CoreCell(Agent):
    def __init__(self, model, pos, resource_amount=20):
        super().__init__(model)
        self.pos = pos
        self.resource_amount = resource_amount
        self.direct_opposite_chance = 0.3
        self.broad_opposite_chance = 0.1
        self.state = ALIVE

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
        neighbours = [
            cell for cell in self.pos.neighborhood
            if any(isinstance(obj, CoreCell) and obj.resource_amount < self.resource_amount
                   for obj in cell.agents)
        ]

        if not neighbours:
            return
        amount = self.share()
        amount = np.floor(amount / len(neighbours))

        for neighbour in neighbours:
            neighbour.resource_amount += amount

    def use_resource(self):
        self.resource_amount -= self.model.resource_consumption

    def decide_development(self):
        if self.state == DEAD:
            return
        # Get empty neighboring cells
        empty_cells = [
            cell for cell in self.pos.neighborhood
            if not any(isinstance(obj, CoreCell) for obj in cell.agents)
        ]

        if not empty_cells:
            return  # No available space to expand

        # Calculate probabilities for each empty cell
        probabilities = []
        for cell in empty_cells:
            prob = 1.0  # Base probability
            for neighbor in cell.neighborhood:
                if any(isinstance(obj, CoreCell) for obj in neighbor.agents):
                    prob += self.direct_opposite_chance
            probabilities.append(prob)

        # Normalize probabilities
        total = sum(probabilities)
        probabilities = [p / total for p in probabilities]

        # Select a cell based on probabilities
        chosen_cell = random.choices(empty_cells, weights=probabilities, k=1)[0]

        # Share resource and spawn new core
        shared_resource = self.share()
        if shared_resource > 0:
            new_cell = CoreCell(self.model, chosen_cell, shared_resource)
            self.model.register_agent(new_cell)

    def feed(self):
        if self.state == DEAD:
            return

        available = self.pos.resource
        if available > 0:
            harvested = min(10, available)
            self.pos.resource -= harvested
            self.resource_amount += harvested

        self.share_with_neighbour()

        if self.resource_amount <= 1:
            self.state = DEAD
        self.use_resource()
