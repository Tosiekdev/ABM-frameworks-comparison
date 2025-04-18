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
    agh::Network<AgentNode, char> network;

    float a, b, c;
};
