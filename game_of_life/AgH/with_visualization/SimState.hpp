#pragma once

#include "Agent.hpp"

#include <model/Model.hpp>
#include <space/Field.hpp>

#include <experimental/visualization/FieldPortrayal.hpp>

struct SimState : agh::Model<Agent> {
    SimState(int width, int height, int epochs, std::string_view file);
    agh::Field<Agent> space;
    void beforeStep();
    void afterStep();
    bool shouldEnd(int epochs);

    int duration;
    agh::FieldPortrayal<Agent> portrayal;
};
