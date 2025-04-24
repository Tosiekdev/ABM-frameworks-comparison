#pragma once

#include "CoreCell.hpp"
#include "ExplorationCell.hpp"

#include <model/Model.hpp>
#include <space/Field.hpp>
#include <space/ValueLayer.hpp>
#include <utilities/Concepts.hpp>

struct Scheduler;

struct FungiModel : agh::Model<CoreCell, ExplorationCell> {
    explicit FungiModel(Scheduler& s, int width, int height, int max);
    void init(int numOfCells);

    void beforeStep() {}

    void afterStep();

    bool shouldEnd(int e);

    agh::Field<CoreCell, ExplorationCell> fungus;
    agh::ValueLayer<int> resources;
    Scheduler& schedule;
    int epochs{};
    int maxEpochs;
    int resourceConsumption = 1;

private:
    void genResources();
    void genCoreCells(int num);
    void getStats();

    static constexpr int resourceR = 10;
    static constexpr int resourceNum = 20;

    static constexpr int minResources = 20;
    static constexpr int maxResources = 40;
};

static_assert(agh::SimState<FungiModel>);
