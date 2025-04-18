#pragma once

#include "Cell.hpp"

#include <optional>
#include <space/Point.hpp>

struct FungiModel;

struct ExplorationCell : Cell {
    explicit ExplorationCell(const int res=10) : Cell(res) {}

    void step(FungiModel& model);

    void advance(FungiModel& model);

    void decideExp(FungiModel& model);

    void feed(FungiModel& model);

    bool isLandlocked{false};

private:
    static int random();

    static float rrandom();

    static constexpr float directOppositeChance = 0.9f;
    static constexpr float broadOppositeChance = 0.01f;
};
