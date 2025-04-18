import random
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
    def step(self):
        pass
