#pragma once

#include "CoreCell.hpp"
#include "ExplorationCell.hpp"

#include <model/Model.hpp>
#include <space/Field.hpp>
#include <space/ValueLayer.hpp>
#include <utilities/Concepts.hpp>

struct FungiModel : agh::Model<CoreCell, ExplorationCell> {
    explicit FungiModel(): fungus(10,10), resources(10,10) {}

    void beforeStep();
    void afterStep();
    bool shouldEnd(int epochs);

    agh::Field<CoreCell, ExplorationCell> fungus;
    agh::ValueLayer<int> resources;
};

static_assert(agh::SimState<FungiModel>);
