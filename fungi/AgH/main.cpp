#include <iostream>

#include "FungiModel.hpp"
#include "Scheduler.hpp"

int main() {
    Scheduler s;
    FungiModel model(s, 10, 10, 100);
    agh::Schedule<FungiModel, CoreCell, ExplorationCell> schedule(model);

    s.v = &schedule;

    model.init(1);
    schedule.execute();

    return 0;
}
