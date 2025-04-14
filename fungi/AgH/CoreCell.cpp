#include "CoreCell.hpp"
#include "FungiModel.hpp"

void CoreCell::step(FungiModel& model) {

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

void CoreCell::share() {}

void CoreCell::useResources() {}
