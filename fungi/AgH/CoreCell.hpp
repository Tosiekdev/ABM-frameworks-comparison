#pragma once

#include <optional>
#include <space/Point.hpp>

struct FungiModel;

struct CoreCell {
    explicit CoreCell(const float initResources) : resources(initResources) {};

    void step(FungiModel& model);

    void advance(FungiModel& model);

    [[nodiscard]] bool isActive() const {
        return alive;
    }

    void addResources(float res);

    std::optional<agh::Point> pos;

private:
    void share();
    void useResources();
    static int random();
    static float rrandom();

    bool alive{true};
    float resources;
    float consumption{1.f};

    static constexpr float directOppositeChance = 0.3f;
    static constexpr float broadOppositeChance = 0.1f;

    static constexpr float maxNgh = 0.3f;
    static constexpr float safeNgh = 0.f;
};
