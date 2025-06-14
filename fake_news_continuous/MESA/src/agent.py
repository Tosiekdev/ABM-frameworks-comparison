import random

from .distributions import (State,
                            EducationDistribution,
                            SexDistribution,    
                            PlatformAgeDistribution)
from mesa.experimental.continuous_space import ContinuousSpaceAgent
import numpy as np


class FakeNewsAgent(ContinuousSpaceAgent):
    NN_DISCTANCE = 20
    MN_DISTANCE = 10
    MAX_MOVE = 2

    """Individual Agent definition and its properties/interaction methods."""
    def __init__(self, space, model, state, position):
        super().__init__(space, model)
        self.state = state
        self.next_state = state
        self.age = self.weighted_choice(PlatformAgeDistribution[state])
        self.education = self.weighted_choice(EducationDistribution[state])
        self.sex = self.weighted_choice(SexDistribution[state])
        self.position = position

    def weighted_choice(self, distribution):
        options, weights = zip(*distribution.items())
        return random.choices(options, weights=weights, k=1)[0]

    def compute_transition_score(self, transition):
        neighbors = self.get_neighbors_in_radius(self.NN_DISCTANCE)[0]
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

    def move(self):
        nghs = self.get_neighbors_in_radius(self.MN_DISTANCE)[0]

        if len(nghs) == 0:
            return
        
        filtered_nghs = list(filter(lambda x : x.state == self.state, nghs))
        if len(filtered_nghs) == 0:
            return
        
        new_pos = np.sum(np.array([ngh.position for ngh in filtered_nghs]), axis=0) / len(filtered_nghs)
        dx = new_pos[0] - self.position[0]
        dy = new_pos[1] - self.position[1]
        dist = np.sqrt(dx**2 + dy**2)
        if dist > 0:
            step = self.MAX_MOVE if dist > self.MAX_MOVE else dist
            ratio = step / dist
            dx *= ratio
            dy *= ratio
            self.position = self.position + np.array([dx, dy])


    def advance(self):
        self.state = self.next_state
