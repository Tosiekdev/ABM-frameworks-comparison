package sim.app.Life;

import sim.engine.SimState;
import sim.engine.Steppable;

public class Swapper implements Steppable {

    public void step(SimState state) {
        LifeModel model = (LifeModel) state;
        for (LifeAgent agent : model.agents) {
            model.grid.set(agent.getX(), agent.getY(), (agent.changeState() ? 1 : 0));
        }
    }
}
