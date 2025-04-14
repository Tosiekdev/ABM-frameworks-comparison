#pragma once

#include <optional>
#include <space/Point.hpp>

struct FungiModel;

struct CoreCell {
    void step(FungiModel& model);

    void advance(FungiModel& model);

    [[nodiscard]] bool isActive() const {
        return alive;
    }

    std::optional<agh::Point> pos;

private:
    bool alive;

    static constexpr float directOppositeChance = 0.3f;
};
