#pragma once

#include <optional>
#include <space/Point.hpp>

struct FungiModel;

struct Cell {
    explicit Cell(const int res) : resources(res) {}

    [[nodiscard]] bool isActive() const {
        return alive;
    }

    int share();

    void shareWithNghs(FungiModel& model);

    void feed(FungiModel& model);

    void useResources(FungiModel& model);

    std::optional<agh::Point> pos;
    bool alive = true;
    int resources;
};
