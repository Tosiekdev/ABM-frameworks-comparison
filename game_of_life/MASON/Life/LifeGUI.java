package sim.app.Life;
import sim.engine.*;
import sim.display.*;
import sim.portrayal.grid.*;
import java.awt.*;
import javax.swing.*;

public class LifeGUI extends GUIState
{
    public Display2D display;
    public JFrame displayFrame;
    String[] modelArgs;

    public LifeGUI() { super(new LifeModel(System.currentTimeMillis())); }

    public LifeGUI(String[] args) { super(new LifeModel(System.currentTimeMillis(), args)); }

    public LifeGUI(SimState state) { super(state); }

    public static String getName() { return "Game of Life"; }


    FastValueGridPortrayal2D gridPortrayal = new FastValueGridPortrayal2D();

    public void setupPortrayals()
    {
        gridPortrayal.setField(((LifeModel)state).grid);
        gridPortrayal.setMap(
                new sim.util.gui.SimpleColorMap(
                        new Color[] {new Color(0,0,0,0), Color.blue}));
    }

    public void start()
    {
        super.start();
        setupPortrayals();
        display.reset();
        display.repaint();
    }

    public void init(Controller c)
    {
        super.init(c);

        LifeModel tut = (LifeModel) state;
        display = new Display2D(tut.gridWidth * 4, tut.gridHeight * 4,this);
        displayFrame = display.createFrame();
        c.registerFrame(displayFrame);
        displayFrame.setVisible(true);


        display.attach(gridPortrayal,"Life");
        display.setBackdrop(Color.black);
    }

    public static void main(String[] args)
    {
        LifeGUI tutorial2 = new LifeGUI();
        Console c = new Console(tutorial2);
        c.setVisible(true);
    }

    public void load(SimState state)
    {
        super.load(state);
        setupPortrayals();
        display.reset();
        display.repaint();
    }

}




