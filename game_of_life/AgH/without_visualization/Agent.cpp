#include "Agent.hpp"

#include <iostream>

#include "SimState.hpp"

Agent::Agent(const bool isAlive) : alive(isAlive) {}

void Agent::step(SimState& model) {
    std::vector<std::variant<Agent*>> nghs = model.space.getNeighbors(*pos);
    const size_t aliveNeighbours = std::ranges::count_if(nghs,
                                                         [](const std::variant<Agent*>& a) {
                                                             return std::get<Agent*>(a)->alive;
                                                         });
    if (alive) {
        futureState = aliveNeighbours == 2 || aliveNeighbours == 3;
    } else {
        futureState = aliveNeighbours == 3;
    }
}

void Agent::advance(SimState&) {
    alive = futureState;
}
