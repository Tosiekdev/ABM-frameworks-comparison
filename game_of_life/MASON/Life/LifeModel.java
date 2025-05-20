package sim.app.Life;

import sim.engine.*;
import sim.field.grid.*;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class LifeModel extends SimState
{
    //TODO: reading parameters
    public LifeModel(long seed)
    {
        super(seed);
    }
    public IntGrid2D grid;

    public static final int gridWidth = 100;
    public static final int gridHeight = 100;
    Random rand = new Random();
    boolean state;
    List<LifeAgent> agents = new ArrayList<>();

    void fillGrid()
    {
        for(int i=0;i< gridWidth;i++)
            for(int j=0;j<gridHeight;j++){
                state = rand.nextBoolean();
                LifeAgent agent = new LifeAgent(i,j,state);
                agents.add(agent);
                grid.set(i, j, (state ? 1 : 0));
                schedule.scheduleRepeating(agent);
            }
        schedule.scheduleRepeating(new Swapper());
    }

    public void start()
    {
        super.start();

        //IMPORTANT!!!
        schedule.setShuffling(false);

        grid = new IntGrid2D(gridWidth, gridHeight);
        fillGrid();
    }

    public static void main(String[] args)
    {
        doLoop(LifeModel.class, args);
        System.exit(0);
    }


}