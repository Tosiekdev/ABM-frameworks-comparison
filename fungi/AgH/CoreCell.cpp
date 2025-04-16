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
        const int childResources = resources;
        const int dir = random();
        resources = 0;
        const agh::Point childPos{pos->x + x(DIRECTIONS[dir]), pos->y + y(DIRECTIONS[dir])};

        if (model.fungus.isEmpty(childPos)) {
            // if no neighbours, add random one
            auto& cell = model.emplaceAgent<CoreCell>(childResources);
            model.fungus.addAgent(cell, childPos);
            model.schedule.v.scheduleRepeating(cell, model.epochs+3, 1, 3);
        } else {
            // or add resources to the existing
            std::visit([=](auto c){ c->addResources(resources); }, *model.fungus.getAgent(childPos));
        }
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
}

void CoreCell::advance(FungiModel& model) {
    share();

    if (const int available = model.resources.get(*pos); available) {
        const int harvested = std::min(10, available);
        model.resources.setOnRead(*pos, available-harvested);
        resources += harvested;
    }

    if (resources < 1) {
        alive = false;
    }

    resources -= consumption;
}

void CoreCell::addResources(const int res) {
    resources += res;
}

int CoreCell::share() {
    if (resources <= 3) {
        return 0;
    }

    if (resources < 10) {
        const int share = resources / 2;
        resources -= share;
        return share;
    }

    const int share = static_cast<int>(resources * 0.8);
    resources -= share;
    return share;
}

void CoreCell::useResources() {}

int CoreCell::random() {
    static std::random_device rand;
    static std::mt19937 rng(rand());
    static std::uniform_int_distribution distribution(1, 8);
    return distribution(rng);
}
