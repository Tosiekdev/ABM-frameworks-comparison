#pragma once

#include "Cell.hpp"

#include <optional>
#include <space/Point.hpp>

struct FungiModel;

struct CoreCell : Cell {
    explicit CoreCell(const int res) : Cell(res) {};

    void step(FungiModel& model);

    void advance(FungiModel& model);

    [[nodiscard]] bool isActive() const {
        return alive;
    }

    void decideDev(FungiModel& model);

private:
    static int random();

    static float rrandom();

    static constexpr float directOppositeChance = 0.3f;
    static constexpr float broadOppositeChance = 0.1f;
};
