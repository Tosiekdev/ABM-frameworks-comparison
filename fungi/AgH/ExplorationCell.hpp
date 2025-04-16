#pragma once

#include <optional>
#include <space/Point.hpp>

struct FungiModel;

struct ExplorationCell {
    void step(FungiModel& model);

    [[nodiscard]] bool isActive() const {
        return alive;
    }

    std::optional<agh::Point> pos;

private:
    bool alive;

    static constexpr float directOppositeChance = 0.9f;
    static constexpr float broadOppositeChance = 0.01f;

    static constexpr float maxNgh = 0.2f;
    static constexpr float safeNgh = 0.1f;
};
