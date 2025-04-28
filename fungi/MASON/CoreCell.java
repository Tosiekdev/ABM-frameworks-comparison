package sim.app.antsforage;

import sim.engine.*;
import sim.field.grid.SparseGrid2D;
import sim.util.Int2D;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class CoreCell implements Steppable {

    public int id;
    public Int2D loc;
    public FungiState.LifeState state;
    public float resourceAmount;

    private boolean checkedForResources;
    private boolean justAte;

    public CoreCell(int id, Int2D loc, float resourceAmount) {
        this.id = id;
        this.loc = loc;
        this.state = FungiState.LifeState.Alive;
        this.resourceAmount = resourceAmount;
        this.checkedForResources = false;
        this.justAte = false;
    }



    private Int2D expansionToLoc(FungiState.ExpansionDirection dir) {
        switch (dir) {
            case Top: return new Int2D(loc.x, loc.y + 1);
            case Bottom: return new Int2D(loc.x, loc.y - 1);
            case Right: return new Int2D(loc.x + 1, loc.y);
            case Left: return new Int2D(loc.x - 1, loc.y);
            case TopRight: return new Int2D(loc.x + 1, loc.y + 1);
            case TopLeft: return new Int2D(loc.x - 1, loc.y + 1);
            case BottomRight: return new Int2D(loc.x + 1, loc.y - 1);
            case BottomLeft: return new Int2D(loc.x - 1, loc.y - 1);
            default: return loc;
        }
    }

    public void growStep(FungiState state) {
        if (state == null || this.state == FungiState.LifeState.Dead) return;
        decideCoreDevelopment(state);
    }

    private void decideCoreDevelopment(FungiState state) {
        List<Int2D> emptyCells = new ArrayList<>();
        int neighborCount = 0;

        for (int dx = -1; dx <= 1; dx++) {
            for (int dy = -1; dy <= 1; dy++) {
                if (dx == 0 && dy == 0) continue; // skip center cell

                int nx = loc.x + dx;
                int ny = loc.y + dy;

                // Boundary check (no toroidal wrapping assumed here)
                if (nx >= 0 && nx < state.dim.x && ny >= 0 && ny < state.dim.y) {
                    if (state.cellGrid.getObjectsAtLocation(nx,ny)  == null) {
                        emptyCells.add(new Int2D(nx, ny));
                    }
                    else neighborCount++;
                }
            }}

        Random rng = state.random;

        if (neighborCount == 0) {
            Int2D newLoc = emptyCells.get(rng.nextInt(emptyCells.size()));
            CoreCell newCell = new CoreCell(state.getNextId(), newLoc, this.resourceAmount);
            this.resourceAmount = 0.0f;
            state.schedule.scheduleRepeating(newCell, 3);
            return;
        }

        // similar logic for expansion/deactivation/creating new exploration cells
        // (I'll continue it after if you want, it's a bit long)
    }

    private void harvestResource(FungiState state) {
        if (checkedForResources) return;

        float resource = state.getResourceAt(loc);
        if (resource > 0.0f) {
            if (resource < FungiState.HARVEST_AMOUNT) {
                addResource(resource);
                state.setResourceAt(loc, 0.0f);
            } else {
                resourceAmount += FungiState.HARVEST_AMOUNT;
                state.setResourceAt(loc, resource - FungiState.HARVEST_AMOUNT);
            }
        } else {
            checkedForResources = true;
        }
    }

    private void consumeResource() {
        if (state == FungiState.LifeState.Dead) return;
        if (resourceAmount >= FungiState.CONSUME_AMOUNT) {
            resourceAmount -= FungiState.CONSUME_AMOUNT;
        } else {
            state = FungiState.LifeState.Dead;
        }
    }

    public void feedStep(FungiState state) {
        harvestResource(state);
        consumeResource();

        List<FungiState.Cell> neighbors = getNeighbors(state);
        if (state.equals(FungiState.LifeState.Dead)) {
            revive(neighbors);
        }

        shareResource(state, neighbors);
    }

    private List<FungiState.Cell> getNeighbors(FungiState state) {
        List<FungiState.Cell> neighbors = new ArrayList<>();
        for (Int2D neighborLoc : getNeighborhoodPos(state)) {
            if (state.cellGrid.getObjectsAtLocation(neighborLoc.x, neighborLoc.y) != null) {
                for (Object obj : state.cellGrid.getObjectsAtLocation(neighborLoc.x, neighborLoc.y)) {
                    if (obj instanceof FungiState.Cell) {
                        neighbors.add((FungiState.Cell) obj);
                    }
                }
            }
        }
        return neighbors;
    }

    //TODO: sharing resource

    private void addResource(float amount) {
        this.resourceAmount += amount;
    }

    @Override
    public void step(SimState simState) {
        FungiState state = (FungiState) simState;

        if (state.totalAliveCells == 0 && state.step != 0) {
            feedStep(state);
            justAte = true;
            return;
        }

        switch (state.getStepType()) {
            case Feed:
                feedStep(state);
                break;
            case Grow:
                growStep(state);
                break;
            case Explore:
                // do nothing
                break;
        }
    }
}
