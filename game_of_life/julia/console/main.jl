using Pkg
Pkg.activate(@__DIR__)

using Agents, Random


using Dates  # do pomiaru czasu
using Observables


if length(ARGS) < 3
    error("Użycie: include(\"main.jl\") <steps> <scenario_file> <grid_size>")
end

max_iter = parse(Int, ARGS[1])
scenario_file = ARGS[2]
grid_size = parse(Int, ARGS[3])

iter_count = 0
time_start_global = nothing  # globalny czas startu symulacji

rules = (1, 3, 3, 4) # (D, S, R, O)

mutable struct Cell <: AbstractAgent
    id::Int
    pos::Dims{2}
    status::Bool
end

function build_model(; rules::Tuple, dims = (100, 100), metric = :chebyshev, seed = 120, initial_status = nothing)
    space = GridSpace(dims; metric)
    properties = Dict(:rules => rules)
    rng = MersenneTwister(seed)

    model = ABM(
        Cell, space;
        properties,
        rng = rng,
        agent_step! = dummystep,
        model_step! = ca_step!
    )

    idx = 1
    for x in 1:dims[1]
        for y in 1:dims[2]
            status = false
            if initial_status !== nothing
                status = initial_status[x, y]
            end
            add_agent_own_pos!(Cell(idx, (x, y), status), model)
            idx += 1
        end
    end

    return model, rng
end


function ca_step!(model)
    new_status = fill(false, nagents(model))
    for agent in allagents(model)
        n = alive_neighbors(agent, model)
        if agent.status == true && (n ≤ model.rules[4] && n ≥ model.rules[1])
            new_status[agent.id] = true
        elseif agent.status == false && (n ≥ model.rules[3] && n ≤ model.rules[4])
            new_status[agent.id] = true
        end
    end

    for id in allids(model)
        model[id].status = new_status[id]
    end
end

function alive_neighbors(agent, model)
    c = 0
    for n in nearby_agents(agent, model)
        if n.status
            c += 1
        end
    end
    return c
end

# Funkcja do wykonania jednego kroku symulacji z liczeniem iteracji i czasem
function do_step!(model)
    global iter_count, time_start_global
    if iter_count == 0
        time_start_global = now()  # start mierzenia czasu przy pierwszym kroku
    end

    if iter_count < max_iter
        ca_step!(model)
        global iter_count += 1
        return true
    else
        # koniec iteracji - mierzymy czas i wypisujemy
        time_end_global = now()
        total_time = time_end_global - time_start_global
        println("Total time for $max_iter steps: ", total_time)
        return false
    end
end

function read_scenario(file, grid_size)
    lines = readlines(file)
    # Zakładamy, że plik ma grid_size linii, każda linia ma grid_size znaków 0/1
    # Możesz dodać sprawdzenie, czy linie i długość się zgadza
    if length(lines) != grid_size
        error("Liczba linii w pliku nie zgadza się z grid_size")
    end

    scenario = Array{Bool}(undef, grid_size, grid_size)
    for (i, line) in enumerate(lines)
        if length(line) != grid_size
            error("Długość linii $i nie zgadza się z grid_size")
        end
        for j in 1:grid_size
            scenario[i, j] = line[j] == '1'
        end
    end
    return scenario
end



initial_status = read_scenario(scenario_file, grid_size)

# Zbuduj model z wczytanym stanem
model, rng = build_model(rules=rules, dims=(grid_size, grid_size), initial_status=initial_status)

function run_simulation()
    #println()
    my_step = 0
    while do_step!(model)
    end
end

run_simulation()

