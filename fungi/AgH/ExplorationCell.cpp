#include "ExplorationCell.hpp"
#include "FungiModel.hpp"

#include <random>

void ExplorationCell::step(FungiModel& model) {
    if (isLandlocked) {
        alive = false;
        return;
    }
}

void ExplorationCell::advance(FungiModel& model) {
    share(model);

    if (const float available = model.resources.get(*pos); available > 0.f) {
        const float harvested = std::min(10.f, available);
        model.resources.setOnRead(*pos, available - harvested);
        resources += harvested;
    }

    useResources();
}

void ExplorationCell::addResources(const float res) {
    resources += res;
}

float ExplorationCell::getResources() const {
    return resources;
}

void ExplorationCell::share(FungiModel& model) {
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

void ExplorationCell::useResources() {
    if (resources < 1.f) {
        alive = false;
    }

    resources -= consumption;
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

std::optional<agh::Point> ExplorationCell::seesResources(const FungiModel& model) const {
    const std::vector<agh::Point> nghs = model.resources.getNeighborhood(*pos, 1, true, false);
    for (auto p : nghs) {
        if (model.resources.get(p) > 0.f) {
            return p;
        }
    }

    return std::nullopt;
}
