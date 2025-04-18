#include "Cell.hpp"
#include "FungiModel.hpp"

namespace {
    struct CmpRes {
        int resources;

        bool operator()(const CoreCell* cell) const {
            return cell->resources >= resources;
        }

        bool operator()(ExplorationCell* cell) const {
            return true;
        }
    };
}

int Cell::share() {
    if (resources < 3) {
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

void Cell::shareWithNghs(FungiModel& model) {
    auto nghs = model.fungus.getNeighbors(*pos);
    std::erase_if(nghs, [&](auto c) { return std::visit(CmpRes{resources}, c); });

    if (nghs.empty()) {
        return;
    }

    int amount = share();
    amount /= static_cast<int>(nghs.size());

    for (auto ngh : nghs) {
        std::visit([&](auto c) { c->resources += amount; }, ngh);
    }
}

void Cell::feed(FungiModel& model) {
    if (!alive) {
        return;
    }

    const int available = model.resources.get(*pos);
    if (available > 0) {
        const int harvested = std::min(10, available);
        model.resources.setOnRead(*pos, available - harvested);
        resources += harvested;
    }

    shareWithNghs(model);

    if (resources < 1) {
        alive = false;
    }

    useResources(model);
}

void Cell::useResources(FungiModel& model) {
    resources -= model.resourceConsumption;
}
