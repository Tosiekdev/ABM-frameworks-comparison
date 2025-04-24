#include "ExplorationCell.hpp"

#include <cassert>

#include "FungiModel.hpp"
#include "Scheduler.hpp"

#include <random>

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

        return points.back();
    }
}

void ExplorationCell::step(FungiModel& model) {
    decideExp(model);
}

void ExplorationCell::advance(FungiModel& model) {
    if (model.epochs % 3 == 0) {
        feed(model);
    }
}

void ExplorationCell::decideExp(FungiModel& model) {
    if (isLandlocked) {
        return;
    }

    auto cells = model.fungus.getNeighbors(*pos);
    std::erase_if(cells, [](auto agent) {
        return std::visit([]<typename T>(T*){ return !std::is_same_v<T, ExplorationCell>; }, agent);
    });

    if (cells.size() > 2) {
        isLandlocked = true;
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

    if (model.resources.get(chosenCell) > 0) {
        int sharedRes = share();
        auto& cell = model.emplaceAgent<CoreCell>(sharedRes);
        model.fungus.addAgent(cell, chosenCell);
        model.schedule.v->scheduleRepeating(cell, model.schedule.v->getEpochs() + 4, 1);
    }

    auto& cell = model.emplaceAgent<ExplorationCell>(10);
    model.fungus.addAgent(cell, chosenCell);
    model.schedule.v->scheduleRepeating(cell, model.schedule.v->getEpochs() + 2, 1);
}

void ExplorationCell::feed(FungiModel& model) {
    auto emptyCells = model.fungus.getNeighborhood(*pos);
    std::erase_if(emptyCells, [&](const agh::Point p) {return !model.fungus.isEmpty(p); });

    if (emptyCells.size() >= 7) {
        return;
    }

    const int available = model.resources.get(*pos);
    if (available > 0) {
        const int harvested = std::min(5, available);
        model.resources.setOnRead(*pos, available - harvested);
        resources += harvested;
    }

    shareWithNghs(model);
}

int ExplorationCell::random() {
    static std::random_device rand;
    static std::mt19937 rng(rand());
    static std::uniform_int_distribution distribution(1, 8);
    return distribution(rng);
}

float ExplorationCell::rrandom() {
    static std::random_device rand;
    static std::mt19937 rng(rand());
    static std::uniform_real_distribution distribution(0.f);
    return distribution(rng);
}
