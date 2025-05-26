package sim.app.Life;
import sim.engine.*;
import sim.field.grid.*;
import sim.util.IntBag;

public class LifeAgent implements Steppable
{
    private int x,y;
    private boolean alive, nextState;
    private static final long serialVersionUID = 1;

    public LifeAgent(int x, int y, boolean startingState){
        this.x = x;
        this.y = y;
        this.alive = startingState;
    }

    public boolean isAlive() {
        return this.alive;
    }

    public boolean changeState(){
        this.alive = this.nextState;
        return this.alive;
    }

    public int getX() {return this.x;}
    public int getY() {return this.y;}

    public void step(SimState state)
    {
        LifeModel model = (LifeModel)state;
        //COUNT ALIVE NEIGHBOURS
        IntBag neighbors = model.grid.getMooreNeighbors(this.x,this.y, 1, ObjectGrid2D.TOROIDAL,false);
        int aliveNeighbors = 0;
        for (int i = 0; i < neighbors.size(); i++) {
            aliveNeighbors += neighbors.get(i);
        }

        //CALCULATE NEXT STATE
        this.nextState = this.alive ? (aliveNeighbors === 2 || aliveNeighbors === 3) : aliveNeighbors === 3;
    }
}
