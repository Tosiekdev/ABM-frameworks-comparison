#include "CoreCell.hpp"

#include <random>

#include "Directions.hpp"
#include "FungiModel.hpp"
#include "Scheduler.hpp"

void CoreCell::step(FungiModel& model) {
    std::array<float, 8> probabilities{};
    std::array<bool, 8> presents{};
    int nghs{};

    for (size_t i = 0; i < 8; ++i) {
        presents[i] = !model.fungus.isEmpty({pos->x + x(DIRECTIONS[i]), pos->y + y(DIRECTIONS[i])});
        ++nghs;
    }


    if (!nghs) {
        const float childResources = resources;
        const int dir = random();
        resources = 0;
        const agh::Point childPos{pos->x + x(DIRECTIONS[dir]), pos->y + y(DIRECTIONS[dir])};

        if (model.fungus.isEmpty(childPos)) {
            // if no neighbours, add random one
            auto& cell = model.emplaceAgent<CoreCell>(childResources);
            model.fungus.addAgent(cell, childPos);
            model.schedule.v.scheduleRepeating(cell, model.epochs + 1, 1, 1);
        } else {
            // or add resources to the existing
            std::visit([&](auto c) { c->addResources(resources); }, *model.fungus.getAgent(childPos));
        }
        return;
    }

    for (size_t i = 0; i < 8; ++i) {
        if (!presents[i]) continue;
        probabilities[(i + 4) % 8] += directOppositeChance;
        probabilities[(i + 3) % 8] += broadOppositeChance;
        probabilities[(i + 5) % 8] += broadOppositeChance;
    }

    float deactivationProb = static_cast<float>(nghs) * 0.1f;
    if (deactivationProb > maxNgh) {
        deactivationProb = 0.95f;
    }
    if (deactivationProb <= safeNgh) {
        deactivationProb = 0.01f;
    }
    if (rrandom() < deactivationProb) {
        if (nghs >= 2 || resources < 5) {
            alive = false;
            return;
        }
    }

    for (size_t i = 0; i < 8; ++i) {
        if (!presents[i]) continue;
        const float childResources = resources;
        resources = 0;
        const agh::Point childPos{pos->x + x(DIRECTIONS[(i + 4) % 8]), pos->y + y(DIRECTIONS[(i + 4) % 8])};
        if (model.fungus.isEmpty(childPos)) {
            auto& cell = model.emplaceAgent<ExplorationCell>(childResources);
            model.fungus.addAgent(cell, childPos);
            model.schedule.v.scheduleRepeating(cell, model.epochs + 1, 1);
        } else {
            std::visit([&](auto c) { c->addResources(resources); }, *model.fungus.getAgent(childPos));
        }
        return;
    }

    for (size_t i = 0; i < 8; ++i) {
        if (!presents[i]) {
            probabilities[i] = 0.f;
        }
    }

    const float total = std::accumulate(probabilities.begin(), probabilities.end(), 0.f);

    if (total < 0.1f) {
        alive = false;
        return;
    }

    std::ranges::transform(probabilities, probabilities.begin(), [=](const float p) { return p / total; });
    const float rand = rrandom();
    for (size_t i = 1; i <= 8; ++i) {
        if (rand < std::accumulate(probabilities.begin(), probabilities.end() + i, 0.f)) {
            const float childResources = resources * 0.9f;
            resources -= childResources;
            const agh::Point childPos{pos->x + x(DIRECTIONS[i - 1]), pos->y + y(DIRECTIONS[i - 1])};
            if (model.fungus.isEmpty(childPos)) {
                auto& cell = model.emplaceAgent<ExplorationCell>(childResources);
                model.fungus.addAgent(cell, childPos);
                model.schedule.v.scheduleRepeating(cell, model.epochs + 1, 1);
            } else {
                std::visit([&](auto c) { c->addResources(resources); }, *model.fungus.getAgent(childPos));
            }
            return;
        }
    }

    alive = false;
}

void CoreCell::advance(FungiModel& model) {
    share(model);

    if (const float available = model.resources.get(*pos); available > 0.f) {
        const float harvested = std::min(10.f, available);
        model.resources.setOnRead(*pos, available - harvested);
        resources += harvested;
    }

    useResources();
}

void CoreCell::addResources(const float res) {
    resources += res;
}

float CoreCell::getResources() const {
    return resources;
}

void CoreCell::share(FungiModel& model) {
    if (resources <= 3.f) {
        return;
    }

    float toShare{};

    if (resources < 10.f) {
        toShare = resources / 2.f;
    } else {
        toShare = resources * 0.8f;
    }
    resources -= toShare;

    auto nghs = model.fungus.getNeighbors(*pos);
    float perNgh = toShare / static_cast<float>(nghs.size());
    for (auto ngh : nghs) {
        std::visit([perNgh](auto a){ a->addResources(perNgh); }, ngh);
    }
}

void CoreCell::useResources() {
    if (resources < 1.f) {
        alive = false;
    }

    resources -= consumption;
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
