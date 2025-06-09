#include "FakeNewsModel.hpp"

#include <iostream>
#include <random>

#include "Agent.hpp"
#include "Agent.hpp"

namespace {
    template<int From, int To>
    int rand() {
        static std::random_device rd;
        static std::mt19937 gen(rd());
        static std::uniform_int_distribution<> dist(From, To);
        return dist(gen);
    }
}

FakeNewsModel::FakeNewsModel(const int size, const float paramA, const float paramB,
                             const float paramC,
                             const int maxEpochs) : a(paramA), b(paramB), c(paramC), max(maxEpochs) {
    static constexpr std::array states = {
        Agent::State::S, Agent::State::E, Agent::State::I, Agent::State::D, Agent::State::R
    };
    static constexpr std::array eds = {
        Agent::Education::Primary, Agent::Education::Secondary, Agent::Education::Higher, Agent::Education::Vocational
    };
    static constexpr std::array sexes = {Agent::Sex::Female, Agent::Sex::Male, Agent::Sex::Other};
    static constexpr std::array ages = {
        Agent::AgeGroup::From00to09,
        Agent::AgeGroup::From10to19,
        Agent::AgeGroup::From20to29,
        Agent::AgeGroup::From30to39,
        Agent::AgeGroup::From40to49,
        Agent::AgeGroup::From50to59,
        Agent::AgeGroup::From60to69,
        Agent::AgeGroup::From70to79,
        Agent::AgeGroup::From80toXX,
    };

    for (size_t i = 0; i < size; ++i) {
        emplaceAgent<Agent>(states[rand<0, states.size() - 1>()], eds[rand<0, eds.size() - 1>()],
                            sexes[rand<0, sexes.size() - 1>()], ages[rand<0, ages.size() - 1>()]);
    }

    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<size_t> dist(0, getAgents<Agent>().size()-1);

    for (auto& i: getAgents<Agent>()) {
        for (size_t j = 0; j < getAgents<Agent>().size() / 10; ++j) {
            network.addEdge(i, getAgents<Agent>()[dist(gen)]);
        }
    }
}

void FakeNewsModel::afterStep() {
    std::cout <<
            std::ranges::count_if(getAgents<Agent>(), [](const Agent& agent) {
                return agent.state == Agent::State::S;
            }) << " " <<
            std::ranges::count_if(getAgents<Agent>(), [](const Agent& agent) {
                return agent.state == Agent::State::E;
            }) << " " <<
            std::ranges::count_if(getAgents<Agent>(), [](const Agent& agent) {
                return agent.state == Agent::State::I;
            }) << " " <<
            std::ranges::count_if(getAgents<Agent>(), [](const Agent& agent) {
                return agent.state == Agent::State::D;
            }) << " " <<
            std::ranges::count_if(getAgents<Agent>(), [](const Agent& agent) {
                return agent.state == Agent::State::R;
            }) << std::endl;
}

bool FakeNewsModel::shouldEnd(const int epochs) const {
    return epochs >= max;
}
