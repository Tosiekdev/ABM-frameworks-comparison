#pragma once

#include "CoreCell.hpp"
#include "ExplorationCell.hpp"

#include <model/Model.hpp>
#include <space/Field.hpp>
#include <space/ValueLayer.hpp>
#include <utilities/Concepts.hpp>

struct Scheduler;

struct FungiModel : agh::Model<CoreCell, ExplorationCell> {
    explicit FungiModel(Scheduler& s): fungus(10,10), resources(10,10), schedule(s) {}

    void beforeStep();
    void afterStep();
    bool shouldEnd(int epochs);

    agh::Field<CoreCell, ExplorationCell> fungus;
    agh::ValueLayer<float> resources;
    Scheduler& schedule;
    int epochs;
};

static_assert(agh::SimState<FungiModel>);
