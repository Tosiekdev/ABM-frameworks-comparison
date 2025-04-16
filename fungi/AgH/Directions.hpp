#pragma once

#include <array>
#include <cstdint>

enum class Direction : uint8_t {
    LEFT,
    TOP_LEFT,
    TOP,
    TOP_RIGHT,
    RIGHT,
    DOWN_RIGHT,
    DOWN,
    DOWN_LEFT
};

constexpr std::array DIRECTIONS = {
    Direction::LEFT, Direction::TOP_LEFT, Direction::TOP, Direction::TOP_RIGHT,
    Direction::RIGHT, Direction::DOWN_RIGHT, Direction::DOWN, Direction::DOWN_LEFT
};

static constexpr std::array DIRECTIONS_MAP = {
    std::pair{-1, 0}, std::pair{-1, -1}, std::pair{0, -1}, std::pair{1, -1},
    std::pair{1, 0}, std::pair{1, 1}, std::pair{0, 1}, std::pair{1, -1}
};

inline int x(Direction direction) {
    return DIRECTIONS_MAP[static_cast<int>(direction)].first;
}

inline int y(Direction direction) {
    return DIRECTIONS_MAP[static_cast<int>(direction)].second;
}
