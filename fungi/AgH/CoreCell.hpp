#pragma once

#include <optional>
#include <space/Point.hpp>

struct FungiModel;

struct CoreCell {
    explicit CoreCell(const int initResources) : resources(initResources) {};

    void step(FungiModel& model);

    void advance(FungiModel& model);

    [[nodiscard]] bool isActive() const {
        return alive;
    }

    void addResources(int res);

    std::optional<agh::Point> pos;

private:
    int share();
    void useResources();
    static int random();

    bool alive{true};
    int resources;
    int consumption{1};

    static constexpr float directOppositeChance = 0.3f;
    static constexpr float broadOppositeChance = 0.1f;

    static constexpr float maxNgh = 0.3f;
    static constexpr float safeNgh = 0.f;
};
