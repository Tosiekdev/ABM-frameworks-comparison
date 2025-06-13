#pragma once

#include "Agent.hpp"

#include <model/Model.hpp>
#include <space/ContinuousSpace.hpp>

struct FakeNewsModel : agh::Model<Agent> {
    explicit FakeNewsModel(float size, float dx, int nAgents, float paramA, float paramB, float paramC, int maxEpochs);

    agh::ContinuousSpace<Agent> space;

    void beforeStep() {}

    void afterStep();

    [[nodiscard]] bool shouldEnd(int epochs) const;

    float a, b, c;
    int max;

    static constexpr float NN_DISTANCE = 20.f;
    static constexpr float MN_DISTANCE = 10.f;
};
