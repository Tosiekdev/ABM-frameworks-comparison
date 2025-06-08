#pragma once

#include <optional>
#include <space/Point.hpp>

#include <experimental/visualization/FieldPortrayal.hpp>

struct SimState;

struct Agent {
    explicit Agent(bool isAlive);
    [[nodiscard]] bool isActive() { return true; }

    void step(SimState& model);
    void advance(SimState& model);

    bool alive;
    bool futureState{};

    std::optional<agh::Point> pos;

    agh::Color getColor() const;
};
