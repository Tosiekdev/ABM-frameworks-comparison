#include "CoreCell.hpp"

#include <cassert>
#include <iostream>
#include <random>

#include "FungiModel.hpp"
#include "Scheduler.hpp"

namespace {
    float rrandom() {
        static std::random_device rand;
        static std::mt19937 rng(rand());
        static std::uniform_real_distribution distribution(0.f);
        return distribution(rng);
    }

    [[nodiscard]] agh::Point choose(const std::vector<agh::Point>& points, const std::vector<float>& weights) {
        const float rand = rrandom();
        float prob = 0.f;

        for (size_t i = 0; i < weights.size(); ++i) {
            prob += weights[i];
            if (prob > rand) {
                return points[i];
            }
        }
        assert(false && "UNREACHABLE");
    }
}

void CoreCell::step(FungiModel& model) {
    if (model.epochs % 3 != 0) {
        return;
    }

    decideDev(model);
}

void CoreCell::advance(FungiModel& model) {
    feed(model);
}

void CoreCell::decideDev(FungiModel& model) {
    if (!alive) {
        return;
    }

    auto emptyCells = model.fungus.getNeighborhood(*pos);
    std::erase_if(emptyCells, [&](const agh::Point p) {return !model.fungus.isEmpty(p); });

    if (emptyCells.empty()) {
        return;
    }

    std::vector<float> probabilities{};
    for (const auto cell : emptyCells) {
        float prob = 1.f;
        auto nghs = model.fungus.getNeighbors(cell);
        if (!model.fungus.isEmpty(*pos)) {
            prob += directOppositeChance;
        }
        probabilities.push_back(prob);
    }

    float total = std::accumulate(probabilities.begin(), probabilities.end(), 0.f);
    std::ranges::for_each(probabilities, [total](float& p) { p /= total; });

    const agh::Point chosenCell = choose(emptyCells, probabilities);
    const int sharedRes = share();

    if (sharedRes > 0) {
        if (rrandom() < 0.1f) {
            auto& cell = model.emplaceAgent<ExplorationCell>(sharedRes);
            model.fungus.addAgent(cell, chosenCell);
            model.schedule.v->scheduleRepeating(cell, model.schedule.v->getEpochs() + 1, 1);
        }
    }
}

int CoreCell::random() {
    static std::random_device rand;
    static std::mt19937 rng(rand());
    static std::uniform_int_distribution distribution(1, 8);
    return distribution(rng);
}

float CoreCell::rrandom() {
    static std::random_device rand;
    static std::mt19937 rng(rand());
    static std::uniform_real_distribution distribution(0.f);
    return distribution(rng);
}
