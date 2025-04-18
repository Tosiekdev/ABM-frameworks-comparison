#include "FungiModel.hpp"
#include "Scheduler.hpp"

int main() {
    Scheduler s;
    FungiModel model(s, 100, 100, 1000);
    agh::Schedule<FungiModel, CoreCell, ExplorationCell> schedule(model);

    s.v = &schedule;

    model.init(100);
    schedule.execute();

    return 0;
}
