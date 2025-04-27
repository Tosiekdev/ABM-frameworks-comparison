package sim.app.antsforage;

import sim.engine.*;
import sim.field.grid.*;
import sim.util.*;
import java.util.*;
import java.util.stream.Collectors;

public class FungiState extends SimState {

    public long step = 0;
    public int nextId = 1;
    public Int2D dim;
    public DoubleGrid2D resourceGrid;
    public SparseGrid2D cellGrid;
    public int initialNumCells = 1;
    public Map<Int2D, Double> updateCellsResource = new HashMap<>();
    public int totalAliveCells = 0;
    public Vector<Cell> newCells;

    public static final int RESOURCE_CLUSTER_NUM = 10;
    public static final double RESOURCE_MIN_AMOUNT = 5.0;
    public static final double RESOURCE_MAX_AMOUNT = 20.0;
    public static final int RESOURCE_CLUSTER_RADIUS = 5;

    public static final float BROAD_OPPOSITE_CHANCE = 0.1f;
    public static final int CONSUME_AMOUNT = 1;
    public static final float DIRECT_OPPOSITE_CHANCE = 0.3f;
    public static final int HARVEST_AMOUNT = 10;
    public static final int MAX_NEIGHBOR_BEFORE_STARVATION = 7;
    public static final int SAFE_NEIGHBOR = 4;
    
    public enum LifeState {
        Alive,
        Dead
    }

    public enum ExpansionDirection {
        Top, Bottom, Right, Left, TopRight, TopLeft, BottomLeft, BottomRight
    }

    public enum StepType {
        Grow,
        Feed,
        Explore
    }

    public FungiState(long seed, Int2D dim) {
        super(seed);
        this.dim = dim;
        this.resourceGrid = new DoubleGrid2D(dim.x, dim.y, 0.0);
        this.cellGrid = new SparseGrid2D(dim.x, dim.y);
    }

    public int getNextId() {
        return nextId++;
    }

    public Bag getCell(Int2D loc) {
        if (loc.x < 0 || loc.x >= dim.x || loc.y < 0 || loc.y >= dim.y) {
            return null;
        }
        return cellGrid.getObjectsAtLocation(loc.x, loc.y);
    }

    public double getResource(Int2D loc) {
        if (loc.x < 0 || loc.x >= dim.x || loc.y < 0 || loc.y >= dim.y) {
            return 0.0;
        }
        return resourceGrid.get(loc.x, loc.y);
    }

    public StepType getStepType() {
        int s = (int) (step % 5);
        if (s == 0) return StepType.Grow;
        if (s == 1) return StepType.Feed;
        return StepType.Explore;
    }

    public StepType getPrevStepType() {
        if (step == 0) return StepType.Grow;
        int s = (int) ((step - 1) % 5);
        if (s == 0) return StepType.Grow;
        if (s == 1) return StepType.Feed;
        return StepType.Explore;
    }

    @Override
    public void start() {
        super.start();
        reset();
        generateResources();
        generateCoreCells();
        // Plots would be handled in a GUI separately
    }

    public void reset() {
        this.step = 0;
        this.resourceGrid = new DoubleGrid2D(dim.x, dim.y, 0.0);
        this.cellGrid = new SparseGrid2D(dim.x, dim.y);
    }


    public void generateResources() {
        for (int i = 0; i < RESOURCE_CLUSTER_NUM; i++) {
            int x = random.nextInt(RESOURCE_CLUSTER_RADIUS);
            int y = random.nextInt(RESOURCE_CLUSTER_RADIUS);
            int radius = random.nextInt(RESOURCE_CLUSTER_RADIUS);

            for (int j = x - radius; j <= x + radius; j++) {
                for (int k = y - radius; k <= y + radius; k++) {
                    if (j < 0 || k < 0 || j >= dim.x || k >= dim.y) continue;
                    if (resourceGrid.get(j, k) != 0.0) continue;

                    double amount = random.nextDouble() * RESOURCE_MIN_AMOUNT / RESOURCE_MAX_AMOUNT;
                    resourceGrid.set(j, k, amount);
                }
            }
        }
    }

    public void generateCoreCells() {
        for (int id = 0; id < initialNumCells; id++) {
            int x = random.nextInt(dim.x);
            int y = random.nextInt(dim.y);
            Int2D loc = new Int2D(x, y);

            CoreCell cell = new CoreCell(id, loc, 1000.0f);
            cellGrid.setObjectLocation(cell, loc.x, loc.y);
            schedule.scheduleRepeating(cell,3);
        }
    }

    public static void main(String[] args) {
        doLoop(FungiState.class, args);
        System.exit(0);
    }
}
