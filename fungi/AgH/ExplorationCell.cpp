#include "ExplorationCell.hpp"

void ExplorationCell::addResources(const float res) {
    resources += res;
}

float ExplorationCell::getResources() const {
    return resources;
}
