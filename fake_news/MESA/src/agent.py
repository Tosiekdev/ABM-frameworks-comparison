import random

import networkx as nx

from .distributions import (State,
                            AgeGroup,
                            EducationGroup,
                            SexGroup,
                            EducationDistribution,
                            SexDistribution,
                            PlatformAgeDistribution)
from mesa.discrete_space import FixedAgent


class FakeNewsAgent(FixedAgent):
    """Individual Agent definition and its properties/interaction methods."""
    def __init__(
            self,
            model,
            state,
            cell,
    ):
        super().__init__(model)
        self.state = state
        self.next_state = state
        self.cell = cell
        self.age = self.weighted_choice(PlatformAgeDistribution[state])
        self.education = self.weighted_choice(EducationDistribution[state])
        self.sex = self.weighted_choice(SexDistribution[state])

    def weighted_choice(self, distribution):
        options, weights = zip(*distribution.items())
        return random.choices(options, weights=weights, k=1)[0]

    def compute_transition_score(self, transition):
        neighbors = list(self.cell.neighborhood.agents)
        N = len(neighbors) or 1

        counts = {
            State.SUSCEPTIBLE: 0,
            State.EXPOSED: 0,
            State.INFECTED: 0,
            State.DOUBTFUL: 0,
            State.RECOVERED: 0,
        }

        for neighbor in neighbors:
            counts[neighbor.state] += 1

        a, b, c = 1.0, 1.0, 1.0  # Placeholder weights
        base = (
                self.age.value * a * random.uniform(0.9, 1.0) +
                self.education.value * b * random.uniform(0.9, 1.0) +
                self.sex.value * c * random.uniform(0.05, 0.1)
        )

        if transition == (State.SUSCEPTIBLE, State.EXPOSED):
            return base + (counts[State.EXPOSED] + counts[State.INFECTED] - counts[State.DOUBTFUL] - counts[
                State.RECOVERED]) / N
        elif transition == (State.EXPOSED, State.INFECTED):
            return base + (counts[State.EXPOSED] + counts[State.INFECTED] - counts[State.DOUBTFUL] - counts[
                State.RECOVERED]) / N
        elif transition == (State.EXPOSED, State.DOUBTFUL):
            return base - (counts[State.EXPOSED] + counts[State.INFECTED] - counts[State.DOUBTFUL] - counts[
                State.RECOVERED]) / N
        elif transition == (State.INFECTED, State.RECOVERED):
            return base - (counts[State.EXPOSED] + counts[State.INFECTED] - counts[State.DOUBTFUL] - counts[
                State.RECOVERED] + counts[State.SUSCEPTIBLE]) / N
        elif transition == (State.DOUBTFUL, State.EXPOSED):
            return base + (counts[State.EXPOSED] + counts[State.INFECTED] - counts[State.DOUBTFUL] - counts[
                State.RECOVERED] + counts[State.SUSCEPTIBLE]) / N
        return base

    def step(self):
        # Example thresholds
        thresholds = {
            (State.SUSCEPTIBLE, State.EXPOSED): 4.5,
            (State.EXPOSED, State.INFECTED): 4.5,
            (State.EXPOSED, State.DOUBTFUL): 4.5,
            (State.INFECTED, State.RECOVERED): 4.5,
            (State.DOUBTFUL, State.EXPOSED): 4.5,
        }

        if self.state == State.SUSCEPTIBLE:
            score = self.compute_transition_score((State.SUSCEPTIBLE, State.EXPOSED))
            if score > thresholds[(State.SUSCEPTIBLE, State.EXPOSED)]:
                self.next_state = State.EXPOSED

        elif self.state == State.EXPOSED:
            score_i = self.compute_transition_score((State.EXPOSED, State.INFECTED))
            score_d = self.compute_transition_score((State.EXPOSED, State.DOUBTFUL))
            if score_i > thresholds[(State.EXPOSED, State.INFECTED)] and score_i > score_d:
                self.next_state = State.INFECTED
            elif score_d > thresholds[(State.EXPOSED, State.DOUBTFUL)]:
                self.next_state = State.DOUBTFUL

        elif self.state == State.INFECTED:
            score = self.compute_transition_score((State.INFECTED, State.RECOVERED))
            if score > thresholds[(State.INFECTED, State.RECOVERED)]:
                self.next_state = State.RECOVERED

        elif self.state == State.DOUBTFUL:
            score = self.compute_transition_score((State.DOUBTFUL, State.EXPOSED))
            if score > thresholds[(State.DOUBTFUL, State.EXPOSED)]:
                self.next_state = State.EXPOSED

    def rewire(self, remove_prob=0.2, add_prob=0.2):
        G = self.model.grid

        # Don't rewire if this node isn't in the graph
        if self.unique_id not in G:
            return

        nearby = nx.single_source_shortest_path_length(G, self.unique_id, cutoff=2)

        for nid, dist in nearby.items():
            if nid == self.unique_id:
                continue
            if nid not in self.model.agents:
                continue  # just in case
            neighbor = self.model.agents[nid]
            same_state = neighbor.state == self.state
            connected = G.has_edge(self.unique_id, nid)

            if not same_state and connected and self.random.random() < remove_prob:
                G.remove_edge(self.unique_id, nid)

            elif same_state and not connected and self.random.random() < add_prob:
                G.add_edge(self.unique_id, nid)

    def advance(self):
        self.state = self.next_state
