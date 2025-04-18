#pragma once

#include "Cell.hpp"

#include <optional>
#include <space/Point.hpp>

struct FungiModel;

struct ExplorationCell : Cell {
    explicit ExplorationCell(const int res) : Cell(res) {}

    void step(FungiModel& model);

    void advance(FungiModel& model);

    [[nodiscard]] bool isActive() const {
        return alive;
    }

    void addResources(float res);

    [[nodiscard]] float getResources() const;

    std::optional<agh::Point> pos;
    bool alive{true};
    bool isLandlocked{false};

private:
    void share(FungiModel& model);

    void useResources();

    static int random();

    static float rrandom();

    [[nodiscard]] std::optional<agh::Point> seesResources(const FungiModel& model) const;

    float consumption{1.f};

    static constexpr float directOppositeChance = 0.9f;
    static constexpr float broadOppositeChance = 0.01f;

    static constexpr float maxNgh = 0.2f;
    static constexpr float safeNgh = 0.1f;
};
