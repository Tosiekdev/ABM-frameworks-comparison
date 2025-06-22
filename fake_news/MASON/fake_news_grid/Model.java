package sim.app.fake_news;

import sim.engine.*;
import sim.field.grid.*;

public class Model extends SimState
    {
    private static final long serialVersionUID = 1;
    public int gridWidth = 30;
    public int gridHeight = 30;
    public int radius = 2;
    public double a = 0.9;
    public double b = 0.9;
    public double c = 0.9;
    public double alpha = 0.2;
    public double beta = 0.2;
    public double gamma = 0.2;
    public double delta = 0.0;
    public double theta = 0.2;

    public DemographicData demographicData;
    //NOTE: wanted it to be ObjectGrid2D, but this one has getObjectLocation method
    public SparseGrid2D grid;

    public Model(long seed)
        {
        super(seed);
        }

    public void start()
        {
        super.start();
        grid = new SparseGrid2D(gridWidth, gridHeight);
        demographicData = new DemographicData();

        for (int x = 0; x < grid.getWidth(); x++) {
            for (int y = 0; y < gridHeight; y++) {
                Person p = new Person();
                grid.setObjectLocation(p, x, y);
                // Schedule them to step
                schedule.scheduleRepeating(p);
            }
        }
    }

    public static void main(String[] args)
        {
        doLoop(Model.class, args);
        System.exit(0);
        }

    }
