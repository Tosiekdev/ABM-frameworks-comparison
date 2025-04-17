#include "FungiModel.hpp"

#include "Scheduler.hpp"

void FungiModel::afterStep() {
    epochs = static_cast<int>(schedule.v.getEpochs());

    for (auto& agent : getAgents<CoreCell>()) {
        if (!agent.isActive() && agent.getResources() > 5.f) {
            agent.alive = true;
            schedule.v.scheduleRepeating(agent, epochs + 1, 1);
        }
    }

    for (auto& agent : getAgents<ExplorationCell>()) {
        if (!agent.isActive() && agent.getResources() > 5.f) {
            agent.alive = true;
            schedule.v.scheduleRepeating(agent, epochs + 1, 1);
        }
    }
}
