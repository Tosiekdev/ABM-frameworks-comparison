# Game of Life (in Julia)


## Running the Simulation

If you haven't installed Julia, [do that first](https://julialang.org/downloads/).

In folders GUI and CONSOLE you should run following commands:


For gui:


```bash
cd gui
julia --project=.
```

Julia REPL will open run:

```bash
import Pkg
Pkg.activate(".")
Pkg.instantiate()
Pkg.add("Agents")
Pkg.add("Plots") 
Pkg.add("Dates")
Pkg.add("Observables")
Pkg.add("GLMakie")
Pkg.add("CairoMakie")
```


For console"

```bash
cd console
julia --project=.
```

Julia REPL will open run:

```bash
import Pkg
Pkg.activate(".")
Pkg.instantiate()
Pkg.add("Agents")
Pkg.add("Plots") 
Pkg.add("Dates")
Pkg.add("Observables")
Pkg.add("GLMakie")
Pkg.add("CairoMakie")
```



After that, run the `main` program from the root project folder:

For console:

```bash
julia main.jl iterations scenario.txt steps
```


For GUI:

In Julia REPL

```bash
include("main.jl")

```

