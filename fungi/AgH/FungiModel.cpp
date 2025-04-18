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

    for (auto& agent: getAgents<CoreCell>()) {
        if (!agent.isActive() && agent.getResources() > 50.f) {
            agent.alive = true;
            schedule.v->scheduleRepeating(agent, epochs + 1, 1);
        }
    }

    for (auto& agent: getAgents<ExplorationCell>()) {
        if (!agent.isActive() && agent.getResources() > 5.f && !agent.isLandlocked) {
            agent.alive = true;
            schedule.v->scheduleRepeating(agent, epochs + 1, 1);
        }
    }

    getStats();
}

bool FungiModel::shouldEnd(const int e) {
    return e >= maxEpochs;
}

void FungiModel::genResources() {
    std::random_device rand;
    std::mt19937 rng(rand());
    std::uniform_int_distribution rDist(1, resourceR);
    std::uniform_int_distribution xDist(resourceR, resources.getWidth() - resourceR);
    std::uniform_int_distribution yDist(resourceR, resources.getHeight() - resourceR);
    std::uniform_real_distribution resDist(minResources, maxResources);

    for (int n = 0; n < resourceNum; ++n) {
        const int x = xDist(rng);
        const int y = yDist(rng);
        const int r = rDist(rng);

        for (int i = x - r; i < x + r; ++i) {
            for (int j = y - r; j < y + r; ++j) {
                if (resources.get({j, i}) != 0.f) {
                    continue;
                }

                resources.setOnRead({j, i}, resDist(rng));
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
            auto& cell = emplaceAgent<CoreCell>(1000.f);
            fungus.addAgent(cell, pos);
            schedule.v->scheduleRepeating(cell, 1, 1);
        }
    }
}

void FungiModel::getStats() {
    size_t numFungi = getAgents<CoreCell>().size();
    long aliveFungi = std::ranges::count_if(getAgents<CoreCell>(), [](const CoreCell& c) { return c.isActive(); });
    size_t exploration = getAgents<ExplorationCell>().size();
    float totalAgentRes = std::accumulate(getAgents<CoreCell>().begin(), getAgents<CoreCell>().end(), 0.f,
                                          [](const float a, const CoreCell& c) { return a + c.getResources(); });
    float totalGridRes = 0.f;
    resources.apply([&totalGridRes](const float v) { totalGridRes += v; });

    std::cout << "Epoch: " << epochs << " | Fungi: " << numFungi << " | Alive: " << aliveFungi << " | Exploration: " <<
            exploration << " | Resources: " << totalAgentRes << " | Grid: " << totalGridRes << std::endl;
}
