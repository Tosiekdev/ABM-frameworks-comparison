#include "SimState.hpp"

#include <cstdlib>
#include <iostream>
#include <random>
#include <thread>

static std::random_device rd;
static std::mt19937 g(rd());

SimState::SimState(const int width, const int height, const int epochs, const int agentsCount)
    : space(width, height, true), duration(epochs) {
    std::vector<agh::Point> positions = space.getEmpty();
    std::ranges::shuffle(positions, g);
    positions.erase(positions.begin() + agentsCount, positions.end());

    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            if (const agh::Point p{x, y}; std::ranges::find(positions, p) != positions.end()) {
                space.addAgent(emplaceAgent<Agent>(true), p);
            }
            else {
                space.addAgent(emplaceAgent<Agent>(false), p);
            }
        }
    }
}

void SimState::print() {
    system("clear");
    space.transform([&](const agh::Point p, const Agent& a) {
        if (a.alive) {
            std::cout << "\033[1;31m1\033[0m";;
        }
        else {
            std::cout << "0";
        }

        if (p.x == space.getWidth() - 1) {
            std::cout << "\n";
        }
    });
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
}

void SimState::beforeStep() {}

void SimState::afterStep() {
    print();
}

bool SimState::shouldEnd(int epochs) {
    return epochs >= duration;
}
