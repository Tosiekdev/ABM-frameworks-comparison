cmake_minimum_required(VERSION 3.30)
project(AgH)

set(CMAKE_CXX_STANDARD 20)

include_directories(../../Agent-Harmony/include)

add_executable(AgH main.cpp
        FungiModel.cpp
        FungiModel.hpp
        CoreCell.cpp
        CoreCell.hpp
        ExplorationCell.cpp
        ExplorationCell.hpp
        Directions.hpp
        Scheduler.hpp
        Cell.cpp
        Cell.hpp)
