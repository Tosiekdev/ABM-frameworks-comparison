#pragma once

#include "Agent.hpp"

#include <model/Model.hpp>
#include <space/Network.hpp>

struct AgentNode {
    AgentNode(Agent& agent) : agent(agent) {}
    Agent& agent;

    bool operator==(const AgentNode& rhs) const {
        return agent.id == rhs.agent.id;
    }
};

template<>
struct std::hash<AgentNode> {
    size_t operator()(const AgentNode& node) const noexcept {
        return std::hash<Agent>{}(node.agent);
    }
};

struct FakeNewsModel : agh::Model<Agent> {
    explicit FakeNewsModel(int size, float paramA, float paramB, float paramC, int maxEpochs);

    agh::Network<AgentNode, char> network{false};

    void beforeStep() {}

    void afterStep();

    [[nodiscard]] bool shouldEnd(int epochs) const;

    float a, b, c;
    int max;
};
