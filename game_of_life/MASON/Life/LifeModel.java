package sim.app.Life;

import sim.engine.*;
import sim.field.grid.*;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.*;

public class LifeModel extends SimState
{
    public LifeModel(long seed, String[] args) {
        super(seed);
        //CHOOSE INPUT FILE
        String filePath = null;
        List<String> remainingArgsList = new ArrayList<>();

        for (int i = 0; i < args.length; i++) {
            if ("-file".equals(args[i]) && i + 1 < args.length) {
                filePath = args[++i]; // Get the next argument as the file name
            } else {
                remainingArgsList.add(args[i]);
            }
        }
        remainingArgs = remainingArgsList.toArray(new String[0]);


        //READ INPUT FILE
        List<int[]> rows = new ArrayList<>();
        gridHeight = 0;
        try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
            String line;
            while ((line = br.readLine()) != null) {
                // Convert line to array of 0s and 1s
                int[] binaryRow = new int[line.length()];
                gridHeight += 1;
                gridWidth = binaryRow.length;
                for (int i = 0; i < line.length(); i++) {
                    char c = line.charAt(i);
                    if (c == '0' || c == '1') {
                        binaryRow[i] = c - '0';
                    } else {
                        throw new IllegalArgumentException("Invalid character: " + c);
                    }
                }
                rows.add(binaryRow);
            }

            // Convert List<int[]> to int[][]
            inputMatrix = rows.toArray(new int[0][]);


            //IMPORTANT!!!
            schedule.setShuffling(false);
            grid = new IntGrid2D(gridWidth, gridHeight);
        } catch (IOException e) {
            System.err.println("Error reading file: " + e.getMessage());
        }
    }
    public LifeModel(long seed)
    {
        super(seed);
    }
    public IntGrid2D grid;

    public static int gridWidth ;
    public static int gridHeight;
    private static String[] remainingArgs;
    Random rand = new Random();
    boolean state;
    List<LifeAgent> agents = new ArrayList<>();
    static int[][] inputMatrix;


    void fillGrid()
    {

        for(int i=0;i< gridHeight;i++)
            for(int j=0;j<gridWidth;j++){
                state = (inputMatrix[i][j] == 1);
                LifeAgent agent = new LifeAgent(i,j,state);
                agents.add(agent);
                grid.set(j, i, (state ? 1 : 0));
                schedule.scheduleRepeating(agent,1,1);
            }
        schedule.scheduleRepeating(new Swapper(), 10, 1);

    }

    public void start()
    {
        super.start();
        schedule.setShuffling(false);
        agents.clear(); // Clear the agents list to avoid duplicate scheduling
        grid = new IntGrid2D(gridWidth, gridHeight); // Reset the grid state
        fillGrid();
    }

    public static void main(String[] args)
    {
        //START SIMULATION
        doLoop(LifeModel.class, remainingArgs);
        System.exit(0);
    }
}
