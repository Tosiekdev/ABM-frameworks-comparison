import random
import numpy as np
from mesa import Model
from mesa.discrete_space import OrthogonalMooreGrid
from .agent import CoreCell, ExplorationCell
from mesa.discrete_space.property_layer import PropertyLayer


class FungiModel(Model):
    def __init__(self, width, height, initial_fungi=1):
        super().__init__()
        self.num_agents = initial_fungi
        self.random = random.Random()
        self.grid = OrthogonalMooreGrid((width, height), random=self.random, capacity=1)
        self.resource_distribution = self.generate_resource_array(width, height)

        self.direct_opposite_chance = 0.3
        self.broad_opposite_chance = 0.1
        self.resource_consumption = 1

        # Add fungi agents randomly
        CoreCell.create_agents(
            self,
            self.num_agents,
            self.random.choices(self.grid.all_cells.cells, k=self.num_agents),
        )

        # Generate resource clusters
        self.grid.add_property_layer(
            PropertyLayer.from_data("resource", self.resource_distribution)
        )

    @staticmethod
    def generate_resource_array(width, height, num_clusters=20, cluster_radius=10, min_val=20, max_val=40):
        resource_array = np.zeros((width, height), dtype=int)

        for _ in range(num_clusters):
            center_x = random.randint(0, width - 1)
            center_y = random.randint(0, height - 1)
            num_cells = random.randint(5, 15)

            for _ in range(num_cells):
                dx = random.randint(-cluster_radius, cluster_radius)
                dy = random.randint(-cluster_radius, cluster_radius)
                x = center_x + dx
                y = center_y + dy

                if 0 <= x < width and 0 <= y < height:
                    if resource_array[x, y] == 0:
                        resource_array[x, y] = random.randint(min_val, max_val)

        return resource_array

    def step(self):
        fungi_alive = sum(1 for a in self.agents if isinstance(a, CoreCell) and a.state == 1)

        if fungi_alive == 0:
            print("💀 All fungi are dead. Stopping simulation.")
            self.running = False

        self.agents.do("decide_exploration")
        if self.steps % 3 == 0:
            self.agents.do("decide_development")
            self.agents.do("feed")

    def get_stats(self):
        num_fungi = sum(1 for a in self.agents if isinstance(a, CoreCell))
        alive_fungi = sum(1 for a in self.agents if isinstance(a, CoreCell) and a.state == 1)
        exploration = sum(1 for a in self.agents if isinstance(a, ExplorationCell))
        total_agent_resource = sum(a.resource_amount for a in self.agents if isinstance(a, CoreCell))
        total_grid_resource = self.grid.resource.select_cells(condition=lambda data: data > 0, return_list=True)
        return num_fungi, alive_fungi, total_agent_resource, len(total_grid_resource), exploration
