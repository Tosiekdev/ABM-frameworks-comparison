#pragma once

#include "CoreCell.hpp"
#include "ExplorationCell.hpp"

#include <model/Model.hpp>
#include <space/Field.hpp>
#include <space/ValueLayer.hpp>
#include <utilities/Concepts.hpp>

struct Scheduler;

struct FungiModel : agh::Model<CoreCell, ExplorationCell> {
    explicit FungiModel(Scheduler& s, const int max): fungus(10, 10), resources(10, 10), schedule(s), maxEpochs(max) {}

    void beforeStep() {}

    void afterStep();

    bool shouldEnd(int e);

    agh::Field<CoreCell, ExplorationCell> fungus;
    agh::ValueLayer<float> resources;
    Scheduler& schedule;
    int epochs{};
    int maxEpochs;
};

static_assert(agh::SimState<FungiModel>);
