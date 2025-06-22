package sim.app.fake_news_graph;

import sim.app.networktest.CustomNode;
import sim.engine.*;
import sim.field.continuous.Continuous2D;
import sim.field.grid.*;
import sim.field.network.Network;
import sim.util.Double2D;

public class Model extends SimState
    {
    private static final long serialVersionUID = 1;
    public static final  double radius = 30;
    public static final  int population = 60;
    public static final  double a = 0.9;
    public static final  double b = 0.9;
    public static final  double c = 0.9;
    public static final  double alpha = 0.2;
    public static final  double beta = 0.2;
    public static final  double gamma = 0.2;
    public static final  double delta = 0.0;
    public static final  double theta = 0.2;

    public static final double XMIN = 0;
    public static final double XMAX = 800;
    public static final double YMIN = 0;
    public static final double YMAX = 600;

    public static final double DIAMETER = 8;

    public Continuous2D environment = null;
    public Network network = null;

    public DemographicData demographicData;


    public Model(long seed)
        {
        super(seed);
        }

    Person makeNode()
    {
        Person node = new Person();
        environment.setObjectLocation(node, new Double2D(
                random.nextDouble()*(XMAX-XMIN-DIAMETER)+XMIN+DIAMETER/2,
                random.nextDouble()*(YMAX-YMIN-DIAMETER)+YMIN+DIAMETER/2 ) );
        network.addNode(node);
        schedule.scheduleRepeating(node);
        return node;
    }
    public void start()
        {
        super.start();
        environment = new Continuous2D(16.0, (XMAX-XMIN), (YMAX-YMIN) );
        network = new Network();
        demographicData = new DemographicData();

        Person nodes[] = new Person[population];
        for (int i = 0; i <population; i++) {
            nodes[i] = makeNode();
        }
    }

    public static void main(String[] args)
        {
        doLoop(Model.class, args);
        System.exit(0);
        }

    }
