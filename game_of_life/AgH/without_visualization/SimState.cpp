#include "SimState.hpp"

#include <fstream>
#include <iostream>
#include <random>

SimState::SimState(const int width, const int height, const int epochs, std::string_view file)
    : space(width, height, true), duration(epochs) {
    std::ifstream initState(file.data());
    std::string line;
    if (!initState.good()) {
        throw std::runtime_error("Could not open file");
    }

    int y = 0;

    while (std::getline(initState, line)) {
        if (line.empty()) {
            continue;
        }
        for (int x = 0; x < line.size(); ++x) {
            auto& agent = emplaceAgent<Agent>(line[x] == '1');
            space.addAgent(agent, {x, y});
        }
        ++y;
    }
}

void SimState::beforeStep() {}

void SimState::afterStep() {}

bool SimState::shouldEnd(const int epochs) {
    return epochs >= duration;
}
