#pragma once

#include "CoreCell.hpp"
#include "ExplorationCell.hpp"
#include "FungiModel.hpp"

#include <schedule/Schedule.hpp>

struct Scheduler {
    agh::Schedule<FungiModel, CoreCell, ExplorationCell> v;
};