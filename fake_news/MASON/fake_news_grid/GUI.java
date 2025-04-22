/*
  Copyright 2006 by Sean Luke and George Mason University
  Licensed under the Academic Free License version 3.0
  See the file "LICENSE" for more information
*/

package sim.app.fake_news;

import sim.display.Console;
import sim.display.Controller;
import sim.display.Display2D;
import sim.display.GUIState;
import sim.engine.SimState;
import sim.portrayal.SimplePortrayal2D;
import sim.portrayal.grid.FastValueGridPortrayal2D;
import sim.portrayal.grid.SparseGridPortrayal2D;

import javax.swing.*;
import java.awt.*;

public class GUI extends GUIState
    {
    public Display2D display;
    public JFrame displayFrame;

    public GUI() { super(new Model(System.currentTimeMillis())); }
    
    public GUI(SimState state) { super(state); }
    
    public static String getName() { return "Fake News"; }

// We comment this out of the example, which will cause MASON to look
// for a file called "index.html" in the same directory -- which we've
// included for consistency with the other applications in the demo 
// apps directory.   
/* 
   public static Object getInfo()
   {
   return 
   "<H2>Conway's Game of Life</H2>" +
   "<p>... with a B-Heptomino"; 
   }
*/

        SparseGridPortrayal2D gridPortrayal = new SparseGridPortrayal2D();

    public void setupPortrayals() {
        // tell the portrayals what to portray and how to portray them
        gridPortrayal.setField(((Model) state).grid);
        gridPortrayal.setPortrayalForAll(new SimplePortrayal2D() {
            @Override
            public void draw(Object object, Graphics2D graphics, sim.portrayal.DrawInfo2D info) {
                if (object instanceof Person person) {
                    Color color = switch (person.CurrentState) {
                        case SUSCEPTIBLE -> Color.BLUE;
                        case EXPOSED -> Color.ORANGE;
                        case INFECTED -> Color.RED;
                        case DOUBTFUL -> Color.MAGENTA;
                        case RECOVERED -> Color.GREEN;
                    };

                    graphics.setColor(color);
                    graphics.fillOval(
                            (int) (info.draw.x - info.draw.width / 2),
                            (int) (info.draw.y - info.draw.height / 2),
                            (int) (info.draw.width),
                            (int) (info.draw.height)
                    );
                }
            }
        });
    }

    public void start()
        {
        super.start();      
        setupPortrayals();  // set up our portrayals
        display.reset();    // reschedule the displayer
        display.repaint();  // redraw the display
        }
    
    public void init(Controller c)
        {
        super.init(c);
        
        // Make the Display2D.  We'll have it display stuff later.
        Model tut = (Model) state;
        display = new Display2D(tut.gridWidth * 4, tut.gridHeight * 4,this);
        displayFrame = display.createFrame();
        c.registerFrame(displayFrame);   // register the frame so it appears in the "Display" list
        displayFrame.setVisible(true);

        // attach the portrayals
        display.attach(gridPortrayal,"Life");

        // specify the backdrop color  -- what gets painted behind the displays
        display.setBackdrop(Color.black);
        }

    public static void main(String[] args)
        {
        GUI tutorial2 = new GUI();
        Console c = new Console(tutorial2);
        c.setVisible(true);
        }
    
    public void load(SimState state)
        {
        super.load(state);
        setupPortrayals();  // we now have new grids.  Set up the portrayals to reflect this
        display.reset();    // reschedule the displayer
        display.repaint();  // redraw the display
        }
        
    }
    
    
    
    
    
