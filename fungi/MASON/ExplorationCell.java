package sim.app.antsforage;

import sim.app.fake_news_graph.Model;
import sim.engine.*;
import sim.util.Int2D;
import java.util.*;

import static sim.app.antsforage.FungiState.*;

public class ExplorationCell implements Steppable {

    Model model;
    public int id;
    public Int2D loc;
    public FungiState.LifeState state;
    public float resourceAmount;

    private boolean isLandLocked;
    private boolean checkedForResources;
    private boolean justAte;

    public ExplorationCell(int id, Int2D loc, float resourceAmount) {
        this.id = id;
        this.loc = loc;
        this.resourceAmount = resourceAmount;
        this.state = FungiState.LifeState.Alive;
        this.isLandLocked = false;
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

    private Int2D seesResource(FungiState state) {
        for (FungiState.ExpansionDirection dir : FungiState.ExpansionDirection.values()) {
            Int2D neighborLoc = expansionToLoc(dir);
            if (state.getResource(neighborLoc) > 0.0f) {
                return neighborLoc;
            }
        }
        return null;
    }

    private void decideExplorationDevelopment(FungiState state) {
        if (isLandLocked) {
            stateCellDead();
            return;
        }

        Int2D resourceLoc = seesResource(state);
        if (resourceLoc != null) {
            if (state.cellGrid.numObjectsAtLocation(resourceLoc) == 0) {
                float childResource = resourceAmount;
                resourceAmount = 0.0f;
                CoreCell newCore = new CoreCell(state.getNextId(), resourceLoc, childResource);
                state.schedule.scheduleRepeating(newCore, 3);
                return;
            }
        }


        int neighborCount = state.cellGrid.getRadialNeighbors(loc.x,loc.y,1.0,1,false).size();
        float deactivationProb = neighborCount * 0.1f;
        if (deactivationProb > EXPLORATION_MAX_NEIGHBOR_BEFORE_STARVATION) {
            deactivationProb = 0.95f;
        } else if (deactivationProb <= EXPLORATION_SAFE_NEIGHBOR) {
            deactivationProb = 0.01f;
        }

        if (state.random.nextFloat() < deactivationProb) {
            stateCellDead();
            return;
        }

        exploreEmptySpace(state);
    }

    private void exploreEmptySpace(FungiState state) {
        Map<FungiState.ExpansionDirection, Float> probs = new EnumMap<>(FungiState.ExpansionDirection.class);
        for (FungiState.ExpansionDirection dir : FungiState.ExpansionDirection.values()) {
            probs.put(dir, 0.0f);
        }

        for (FungiState.ExpansionDirection dir : FungiState.ExpansionDirection.values()) {
            Int2D locDir = expansionToLoc(dir);
            if (state.cellGrid.getObjectsAtLocation(locDir.x, locDir.y) == null) {
                // Update opposing direction probabilities
                switch (dir) {
                    case Top:
                        probs.put(FungiState.ExpansionDirection.Bottom, probs.get(FungiState.ExpansionDirection.Bottom) + EXPLORATION_DIRECT_OPPOSITE_CHANCE);
                        probs.put(FungiState.ExpansionDirection.BottomLeft, probs.get(FungiState.ExpansionDirection.BottomLeft) + EXPLORATION_BROAD_OPPOSITE_CHANCE);
                        probs.put(FungiState.ExpansionDirection.BottomRight, probs.get(FungiState.ExpansionDirection.BottomRight) + EXPLORATION_BROAD_OPPOSITE_CHANCE);
                        break;
                    // Do same for other directions...
                    // (Left as an exercise, similar to Rust version)
                }
            }
        }

        float totalProb = probs.values().stream().reduce(0.0f, Float::sum);

        if (totalProb < 0.1f) {
            stateCellDead();
            return;
        }

        float pick = state.random.nextFloat() * totalProb;
        float cumSum = 0.0f;
        for (Map.Entry<FungiState.ExpansionDirection, Float> entry : probs.entrySet()) {
            cumSum += entry.getValue();
            if (pick <= cumSum) {
                Int2D newLoc = expansionToLoc(entry.getKey());
                ExplorationCell newCell = new ExplorationCell(state.getNextId(), newLoc, resourceAmount);
                resourceAmount = 0.0f;
                state.schedule.scheduleRepeating(newCell, 1);
                return;
            }
        }
    }

    public void exploreStep(FungiState state) {
        if (state.equals( FungiState.LifeState.Dead)) return;
        decideExplorationDevelopment(state);
    }

	//TODO: harvest and share

    private void stateCellDead() {
        this.state = FungiState.LifeState.Dead;
    }


    private void shareResource(FungiState state, List<Object> neighbors) {
        if (neighbors.isEmpty()) return;
        float shareAmount = getShareAmount();
        if (shareAmount == 0.0f) return;

        float sharePerNeighbor = shareAmount / neighbors.size();
        for (Object neighbor : neighbors) {
            state.neighbor sharePerNeighbor);
        }
    }

    private float getShareAmount() {
        float amount = resourceAmount;
        resourceAmount = 0.0f;
        return amount;
    }

    private void revive(List<Object> neighbors) {
        if (neighbors.size() == 1 && resourceAmount > 0.5f && !isLandLocked || resourceAmount > 5.0f) {
            state = FungiState.LifeState.Alive;
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
                // Do nothing
                break;
            case Explore:
                exploreStep(state);
                break;
        }
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof ExplorationCell)) return false;
        ExplorationCell other = (ExplorationCell) obj;
        return this.id == other.id;
    }

    @Override
    public String toString() {
        return Integer.toString(id);
    }
}
