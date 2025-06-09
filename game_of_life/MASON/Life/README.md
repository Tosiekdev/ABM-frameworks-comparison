# Game Of Life

Simple visualization of the John Conway's Game of Life.  Game of Life is a two-dimensional totalistic cellular automaton, where each cell in the grid simultaneously decides to be "alive" or "dead" based on the following rules:

-If a cell has less than live two neighbors (up/down/left/right -- no diagonal neighbors), is dead in the next iteration, presumably of loneliness.

-If a cell has more than three live neighbors, it is dead in the next iteration from suffucation.

-If a live cell has two or three live neighbors, it stays alive in the next iteration.

-If a dead cell has exactly three live neighbors, it comes alive in the next iteration.

# How to run

OPTION 1 - compile manually

1. To install and run MASON, follow instructions from MASON's official GitHub repository (https://github.com/eclab/mason)

2. Copy entire Life folder to mason/src/main/java/sim/app (you may need to build it after that)

3. Now you can run Game of Life with visualization (LifeGUI) or without (LifeModel).
Both classes take following arguments:

    -file PATH_TO_FILE -for NUMBER_OF_STEPS -time HOW_OFTEN_PRINT_TIMESTAMP

    ...plus all the rest of the arguments you can pass to doLoop function (https://cs.gmu.edu/~eclab/projects/mason/manual.22.pdf).


# OPTION 2 - with IntelliJ IDEA

1. Navigate to:  
   `File` → `Project Structure` → `Project Settings` → `Artifacts` → Click `+` (plus sign) → `Jar` → `From modules with dependencies...`

2. Select a **Main Class** (the one with `main()` method) if you need to make the JAR file runnable.

3. Select **Extract to the target Jar**

4. Click `OK`

5. Click `Apply` / `OK`

> The above sets the "skeleton" to where the JAR file will be saved to.  
> To actually build and save it, do the following:

6. Go to:  
   `Build` → `Build Artifact` → `Build`

7. Try Extracting the `.jar` file from:
    ProjectName  
    └── out  
    └── artifacts  
    └── ProjectName_jar  
    └── ProjectName.jar  

8. Now you can run Game of Life with visualization (LifeGUI) or without (LifeModel).
Both classes take following arguments:

    -file PATH_TO_FILE -for NUMBER_OF_STEPS -time HOW_OFTEN_PRINT_TIMESTAMP

    ...plus all the rest of the arguments you can pass to doLoop function (https://cs.gmu.edu/~eclab/projects/mason/manual.22.pdf).
