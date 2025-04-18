#include "FakeNewsModel.hpp"

#include <schedule/Schedule.hpp>

int main() {
    FakeNewsModel model(100, 0.1f, 0.1f, 0.1f, 100);

    agh::Schedule<FakeNewsModel, Agent> schedule(model);

    for (auto& agent : model.getAgents<Agent>()) {
        schedule.scheduleRepeating(agent, 1, 1);
    }

    schedule.execute();

    return 0;
}
