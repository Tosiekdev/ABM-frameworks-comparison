#pragma once

#include "Agent.hpp"

#include <model/Model.hpp>
#include <space/Field.hpp>

struct SimState : agh::Model<Agent> {
    SimState(int width, int height, int epochs, int agentsCount);
    agh::Field<Agent> space;
    void beforeStep();
    void afterStep();
    bool shouldEnd(int epochs);

    int duration;

    void print();
};
