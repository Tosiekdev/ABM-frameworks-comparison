#include <iostream>

#include "SimState.hpp"

#include <schedule/Schedule.hpp>

int main() {
    SimState state(10, 10, 10, 50);

    agh::Schedule<SimState, Agent> schedule(state);

    for (Agent& agent : state.getAgents<Agent>()) {
        schedule.scheduleRepeating(agent, 1, 1);
    }

    schedule.execute();
}