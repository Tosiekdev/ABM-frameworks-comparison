#include "FungiModel.hpp"

#include <iostream>

#include "Scheduler.hpp"

#include <random>

FungiModel::FungiModel(Scheduler& s, const int width, const int height, const int max) : fungus(width, height),
    resources(width, height), schedule(s),
    maxEpochs(max) {}

void FungiModel::init(const int numOfCells) {
    genResources();
    genCoreCells(numOfCells);
}

void FungiModel::afterStep() {
    epochs = static_cast<int>(schedule.v->getEpochs());
    getStats();
}

bool FungiModel::shouldEnd(const int e) {
    return e >= maxEpochs;
}

void FungiModel::genResources() {
    std::random_device rand;
    std::mt19937 rng(rand());
    std::uniform_int_distribution rDist(-resourceR, resourceR);
    std::uniform_int_distribution xDist(0, resources.getWidth() - 1);
    std::uniform_int_distribution yDist(0, resources.getHeight() - 1);
    std::uniform_int_distribution resDist(minResources, maxResources);
    std::uniform_int_distribution numDist(5, 15);

    for (int n = 0; n < resourceNum; ++n) {
        const int x = xDist(rng);
        const int y = yDist(rng);
        const int numCells = numDist(rng);

        for (int i = 0; i < numCells; ++i) {
            const int dx = rDist(rng);
            const int dy = rDist(rng);

            const agh::Point pos{x + dx, y + dy};
            if (!resources.outOfBounds(pos)) {
                if (resources.get(pos) == 0) {
                    resources.setOnRead(pos, resDist(rng));
                }
            }
        }
    }
}

void FungiModel::genCoreCells(const int num) {
    std::random_device rand;
    std::mt19937 rng(rand());
    std::uniform_int_distribution xDist(0, resources.getWidth() - 1);
    std::uniform_int_distribution yDist(0, resources.getHeight() - 1);

    for (int i = 0; i < num; ++i) {
        const agh::Point pos{xDist(rng), yDist(rng)};
        if (fungus.isEmpty(pos)) {
            auto& cell = emplaceAgent<CoreCell>();
            fungus.addAgent(cell, pos);
            schedule.v->scheduleRepeating(cell, 1, 1);
        }
    }
}

void FungiModel::getStats() {
    const size_t numFungi = getAgents<CoreCell>().size();
    const long aliveFungi = std::ranges::count_if(getAgents<CoreCell>(), [](const CoreCell& c) {
        return c.isActive();
    });
    const size_t exploration = getAgents<ExplorationCell>().size();
    const int totalAgentRes = std::accumulate(getAgents<CoreCell>().begin(), getAgents<CoreCell>().end(), 0,
                                              [](const int a, const CoreCell& c) { return a + c.resources; });
    int totalGridRes = 0;
    for (int i = 0; i < resources.getHeight(); ++i) {
        for (int j = 0; j < resources.getWidth(); ++j) {
            totalGridRes += resources.get({j, i});
        }
    }

    std::cout << "Epoch: " << epochs << " | Fungi: " << numFungi << " | Alive: " << aliveFungi << " | Exploration: " <<
            exploration << " | Resources: " << totalAgentRes << " | Grid: " << totalGridRes << std::endl;
}
