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

    [[nodiscard]] float getResources() const;

    std::optional<agh::Point> pos;
    bool alive{true};

private:
    void share(FungiModel& model);

    void useResources();

    static int random();

    static float rrandom();

    float resources;
    float consumption{1.f};

    static constexpr float directOppositeChance = 0.3f;
    static constexpr float broadOppositeChance = 0.1f;

    static constexpr float maxNgh = 0.3f;
    static constexpr float safeNgh = 0.f;
};
