#include "SimState.hpp"

#include <experimental/visualization/Visualization.hpp>
#include <schedule/Schedule.hpp>

int main(const int argc, char** argv) {
    if (argc != 4) {
        throw std::runtime_error("Invalid number of arguments");
    }

    const int size = std::stoi(argv[3]);
    SimState state(size, size, std::stoi(argv[1]), argv[2]);

    agh::Schedule<SimState, Agent> schedule(state);

    for (Agent& agent : state.getAgents<Agent>()) {
        schedule.scheduleRepeating(agent, 1, 1);
    }
    agh::Visualization vis(1000, 1000, "Game of Life", 1);
    vis.addPortrayal(state.portrayal);

    vis.run(schedule);
}
