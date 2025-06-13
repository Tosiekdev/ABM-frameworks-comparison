cmake_minimum_required(VERSION 3.31)
project(AgH)

set(CMAKE_CXX_STANDARD 20)

include_directories(../../Agent-Harmony/include)

add_executable(AgH main.cpp
        Agent.cpp
        Agent.hpp
        FakeNewsModel.cpp
        FakeNewsModel.hpp
)
