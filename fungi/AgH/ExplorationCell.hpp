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
};
