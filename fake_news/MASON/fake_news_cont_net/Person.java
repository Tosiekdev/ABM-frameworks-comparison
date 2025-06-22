package sim.app.fake_news_graph;

import sim.app.networktest.NetworkTest;
import sim.app.pso3d.Neighborhood3D;
import sim.engine.SimState;
import sim.engine.Steppable;
import sim.field.grid.IntGrid2D;
import sim.field.grid.ObjectGrid2D;
import sim.field.network.Network;
import sim.util.Bag;
import sim.util.Double2D;
import sim.util.Int2D;

import java.util.*;


enum  EducationGroup {
    PRIMARY,
    SECONDARY,
    HIGHER,
    VOCATIONAL
}

enum SexGroup {
    FEMALE,
    MALE,
    OTHER
}

enum AgeGroup{
    from00to09,
    from10to19,
    from20to29,
    from30to39,
    from40to49,
    from50to59,
    from60to69,
    from70to79,
    from80toXX
}

enum State {
    SUSCEPTIBLE,
    EXPOSED,
    INFECTED,
    DOUBTFUL,
    RECOVERED
}


public class Person implements Steppable {
    static double ChanceToConnectSame = 0.8;
    static double ChanceToConnectDiff = 0.4;
    int radius = 2;
    AgeGroup Age;
    SexGroup Gender;
    EducationGroup Education;
    State CurrentState;
    Int2D location;

    public Person() {
        this.Age = AgeGroup.values()[new Random().nextInt(AgeGroup.values().length)];
        this.Gender = SexGroup.values()[new Random().nextInt(SexGroup.values().length)];
        this.Education = EducationGroup.values()[new Random().nextInt(EducationGroup.values().length)];
        this.CurrentState = State.values()[new Random().nextInt(State.values().length)];
    }

    private State CalculateChange (Map<State, Integer> MyNeighbors, Model MyModel, int NumNeighbors) {
        Random r = new Random();
        double change;

        double AgeFactor = MyModel.demographicData.platformAgeDistribution.get(this.CurrentState).get(this.Age) *
                MyModel.a * (0.9 + (0.1) * r.nextDouble());

        double EduFactor = MyModel.demographicData.educationDistribution.get(this.CurrentState).get(this.Education) *
                MyModel.b * (0.9 + (0.1) * r.nextDouble());

        double GenderFactor = MyModel.demographicData.sexDistribution.get(this.CurrentState).get(this.Gender) *
                MyModel.c * (0.5 + (0.5) * r.nextDouble());

        change = AgeFactor + EduFactor + GenderFactor;

        if (NumNeighbors == 0)
            return this.CurrentState;

        double EN = ((double) MyNeighbors.get(State.EXPOSED)) / NumNeighbors;
        double IN = ((double) MyNeighbors.get(State.INFECTED)) / NumNeighbors;
        double DN = ((double) MyNeighbors.get(State.DOUBTFUL)) / NumNeighbors;
        double RN = ((double) MyNeighbors.get(State.RECOVERED)) / NumNeighbors;
        double SN = ((double) MyNeighbors.get(State.SUSCEPTIBLE)) / NumNeighbors;


        switch (CurrentState) {
            case SUSCEPTIBLE: {
                change = change + EN + IN - DN - RN;
                if (change >= MyModel.alpha)
                    return State.EXPOSED;
                else
                    return State.SUSCEPTIBLE;
            }

            case EXPOSED: {
                if ((EN + IN - DN - RN) > (-EN - IN + DN + RN)) {
                    change = change + EN + IN - DN - RN;
                    if (change >= MyModel.beta)
                        return State.SUSCEPTIBLE;
                    else
                        return State.EXPOSED;
                } else {
                    change = change - EN - IN + DN + RN;
                    if (change >= MyModel.gamma)
                        return State.DOUBTFUL;
                    else
                        return State.EXPOSED;
                }
            }

            case INFECTED: {
                change = change - EN - IN + DN + RN - SN;
                if (change >= MyModel.delta)
                    return State.RECOVERED;
                else
                    return State.INFECTED;
            }

            case DOUBTFUL:{
                change = change + EN + IN - DN - RN + SN;
                if (change >= MyModel.theta)
                    return State.EXPOSED;
                else
                    return State.DOUBTFUL;
            }

        }
        return State.RECOVERED;
    }

    @Override
    public void step(SimState state) {
        Random rand = new Random();
        Map<State, Integer> Neighbors = new HashMap<>();
        Neighbors.put(State.DOUBTFUL, 0);
        Neighbors.put(State.SUSCEPTIBLE, 0);
        Neighbors.put(State.INFECTED, 0);
        Neighbors.put(State.EXPOSED, 0);
        Neighbors.put(State.RECOVERED, 0);

        int N = 0;
        float connection;
        Model model = (Model) state;
        Double2D location = model.environment.getObjectLocation(this);
        Network network = model.network;

        //Connect with people around you. Touch some grass.

        Bag neighbors = model.environment.getNeighborsWithinDistance(location, Model.radius);
        for (Object obj : neighbors) {
            if (obj instanceof Person) {
                Person other = (Person) obj;
                connection = (float) rand.nextInt(100) /100;
                if (other.CurrentState == this.CurrentState){
                    if (connection >= ChanceToConnectSame){
                        N += 1;
                        Neighbors.put(this.CurrentState, Neighbors.get(this.CurrentState) + 1);
                        if  (network.getEdge(this, other) == null)
                            network.addEdge(this, other, connection);
                    } else {
                        if  (network.getEdge(this, other) != null)
                            network.removeEdge(network.getEdge(this, other));
                    }
                } else {
                    if (connection >= ChanceToConnectDiff){
                        N += 1;
                        Neighbors.put(this.CurrentState, Neighbors.get(this.CurrentState) + 1);
                        if  (network.getEdge(this, other) == null)
                            network.addEdge(this, other, connection);
                    }  else {
                    if  (network.getEdge(this, other) != null)
                        network.removeEdge(network.getEdge(this, other));
                }
                }
            }
        }

        //Now let's see if our opinions changed
        this.CurrentState = CalculateChange(Neighbors, model, N);
    }
}
