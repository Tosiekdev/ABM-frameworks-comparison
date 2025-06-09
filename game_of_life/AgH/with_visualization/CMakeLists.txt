cmake_minimum_required(VERSION 3.31)
project(AgH)

set(CMAKE_CXX_STANDARD 20)

include_directories(../../../Agent-Harmony/include /home/tosiek/Downloads/SFML-3.0.0/include)
link_directories(../../../Agent-Harmony/build /home/tosiek/Downloads/SFML-3.0.0/lib)


add_executable(AgH main.cpp
        Agent.cpp
        Agent.hpp
        SimState.cpp
        SimState.hpp)
target_link_libraries(AgH sfml-graphics sfml-window sfml-system Agent-Harmony)
