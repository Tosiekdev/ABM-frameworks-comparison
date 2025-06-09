#include "Agent.hpp"
#include "FakeNewsModel.hpp"

#include <map>
#include <random>

int Agent::idCounter = 0;

namespace maps {
    std::map<Agent::State, std::map<Agent::Education, float>> education{
        {
            Agent::State::S, std::map<Agent::Education, float>{
                {Agent::Education::Primary, 0.02},
                {Agent::Education::Secondary, 0.18},
                {Agent::Education::Higher, 0.75},
                {Agent::Education::Vocational, 0.05}
            }
        },
        {
            Agent::State::E, std::map<Agent::Education, float>{
                {Agent::Education::Primary, 0.1},
                {Agent::Education::Secondary, 0.4},
                {Agent::Education::Higher, 0.45},
                {Agent::Education::Vocational, 0.05}
            }
        },
        {
            Agent::State::I, std::map<Agent::Education, float>{
                {Agent::Education::Primary, 0.05},
                {Agent::Education::Secondary, 0.3},
                {Agent::Education::Higher, 0.6},
                {Agent::Education::Vocational, 0.05}
            }
        },
        {
            Agent::State::D, std::map<Agent::Education, float>{
                {Agent::Education::Primary, 0.03},
                {Agent::Education::Secondary, 0.27},
                {Agent::Education::Higher, 0.65},
                {Agent::Education::Vocational, 0.05}
            }
        },
        {
            Agent::State::R, std::map<Agent::Education, float>{
                {Agent::Education::Primary, 0.04},
                {Agent::Education::Secondary, 0.35},
                {Agent::Education::Higher, 0.55},
                {Agent::Education::Vocational, 0.06}
            }
        }
    };

    std::map<Agent::State, std::map<Agent::Sex, float>> sex{
        {
            Agent::State::S, std::map<Agent::Sex, float>{
                {Agent::Sex::Male, 0.55},
                {Agent::Sex::Female, 0.4},
                {Agent::Sex::Other, 0.05}
            }
        },
        {
            Agent::State::E, std::map<Agent::Sex, float>{
                {Agent::Sex::Male, 0.48},
                {Agent::Sex::Female, 0.48},
                {Agent::Sex::Other, 0.4}
            }
        },
        {
            Agent::State::I, std::map<Agent::Sex, float>{
                {Agent::Sex::Male, 0.45},
                {Agent::Sex::Female, 0.5},
                {Agent::Sex::Other, 0.05}
            }
        },
        {
            Agent::State::D, std::map<Agent::Sex, float>{
                {Agent::Sex::Male, 0.50},
                {Agent::Sex::Female, 0.45},
                {Agent::Sex::Other, 0.05}
            }
        },
        {
            Agent::State::R, std::map<Agent::Sex, float>{
                {Agent::Sex::Male, 0.52},
                {Agent::Sex::Female, 0.43},
                {Agent::Sex::Other, 0.05}
            }
        }
    };

    std::map<Agent::State, std::map<Agent::AgeGroup, float>> age = {
        {
            Agent::State::S, {
                {Agent::AgeGroup::From00to09, 0.00f},
                {Agent::AgeGroup::From10to19, 0.05f},
                {Agent::AgeGroup::From20to29, 0.40f},
                {Agent::AgeGroup::From30to39, 0.30f},
                {Agent::AgeGroup::From40to49, 0.15f},
                {Agent::AgeGroup::From50to59, 0.07f},
                {Agent::AgeGroup::From60to69, 0.03f},
                {Agent::AgeGroup::From70to79, 0.00f},
                {Agent::AgeGroup::From80toXX, 0.00f},
            }
        },
        {
            Agent::State::E, {
                {Agent::AgeGroup::From00to09, 0.02f},
                {Agent::AgeGroup::From10to19, 0.10f},
                {Agent::AgeGroup::From20to29, 0.30f},
                {Agent::AgeGroup::From30to39, 0.25f},
                {Agent::AgeGroup::From40to49, 0.15f},
                {Agent::AgeGroup::From50to59, 0.10f},
                {Agent::AgeGroup::From60to69, 0.05f},
                {Agent::AgeGroup::From70to79, 0.02f},
                {Agent::AgeGroup::From80toXX, 0.01f},
            }
        },
        {
            Agent::State::I, {
                {Agent::AgeGroup::From00to09, 0.01f},
                {Agent::AgeGroup::From10to19, 0.26f},
                {Agent::AgeGroup::From20to29, 0.41f},
                {Agent::AgeGroup::From30to39, 0.20f},
                {Agent::AgeGroup::From40to49, 0.07f},
                {Agent::AgeGroup::From50to59, 0.03f},
                {Agent::AgeGroup::From60to69, 0.01f},
                {Agent::AgeGroup::From70to79, 0.01f},
                {Agent::AgeGroup::From80toXX, 0.00f},
            }
        },
        {
            Agent::State::D, {
                {Agent::AgeGroup::From00to09, 0.00f},
                {Agent::AgeGroup::From10to19, 0.05f},
                {Agent::AgeGroup::From20to29, 0.35f},
                {Agent::AgeGroup::From30to39, 0.35f},
                {Agent::AgeGroup::From40to49, 0.15f},
                {Agent::AgeGroup::From50to59, 0.05f},
                {Agent::AgeGroup::From60to69, 0.03f},
                {Agent::AgeGroup::From70to79, 0.02f},
                {Agent::AgeGroup::From80toXX, 0.00f},
            }
        },
        {
            Agent::State::R, {
                {Agent::AgeGroup::From00to09, 0.00f},
                {Agent::AgeGroup::From10to19, 0.10f},
                {Agent::AgeGroup::From20to29, 0.35f},
                {Agent::AgeGroup::From30to39, 0.30f},
                {Agent::AgeGroup::From40to49, 0.15f},
                {Agent::AgeGroup::From50to59, 0.07f},
                {Agent::AgeGroup::From60to69, 0.02f},
                {Agent::AgeGroup::From70to79, 0.01f},
                {Agent::AgeGroup::From80toXX, 0.00f},
            }
        },
    };
}

void Agent::step(FakeNewsModel& model) {
    switch (state) {
        case S: {
            if (StoE(model) > alfa) {
                state = E;
            }
            break;
        }
        case E: {
            const float toI = EtoI(model);
            const float toD = EtoD(model);
            if (toI > beta && toD > gamma) {
                state = toI / beta > toD / gamma ? I : D;
            } else if (toI > beta) {
                state = I;
            } else if (toD > gamma) {
                state = D;
            }
        }
        break;
        case I: {
            if (ItoR(model) > delta) {
                state = R;
            }
        }
        break;
        case D: {
            if (DtoE(model) > theta) {
                state = E;
            }
        }
        break;
        case R:
            ;
    }

    trimConnections(model);
    addConnections(model);
}

float Agent::computeBase(const FakeNewsModel& model) const {
    const float age = maps::age[state][ageGroup];
    const float edu = maps::education[state][education];
    const float gender = maps::sex[state][sex];

    return age * model.a * rand<0.9f, 1.f>() + edu * model.b * rand<0.9f, 1.f>() + gender * model.c * rand<0.05f,
               0.1f>();
}

Agent::Nghs Agent::getNghsCounts(FakeNewsModel& model) {
    using NodeIter = agh::Network<AgentNode, char>::NodeIter;

    const std::vector<NodeIter> nghs = model.network.getNeighborhood(*this);

    const auto N = static_cast<float>(nghs.size());
    const auto E = static_cast<float>(std::ranges::count_if(nghs, [](const NodeIter node) {
        return node->agent.state == State::E;
    }));
    const auto I = static_cast<float>(std::ranges::count_if(nghs, [](const NodeIter node) {
        return node->agent.state == State::I;
    }));
    const auto D = static_cast<float>(std::ranges::count_if(nghs, [](const NodeIter node) {
        return node->agent.state == State::D;
    }));
    const auto R = static_cast<float>(std::ranges::count_if(nghs, [](const NodeIter node) {
        return node->agent.state == State::R;
    }));

    return {N, E, I, D, R};
}

float Agent::StoE(FakeNewsModel& model) {
    const auto [N, E, I, D, R] = getNghsCounts(model);

    return computeBase(model) + E / N + I / N - D / N - R / N;
}

float Agent::EtoI(FakeNewsModel& model) {
    const auto [N, E, I, D, R] = getNghsCounts(model);

    return computeBase(model) + E / N + I / N - D / N - R / N;
}

float Agent::EtoD(FakeNewsModel& model) {
    const auto [N, E, I, D, R] = getNghsCounts(model);

    return computeBase(model) - E / N - I / N + D / N + R / N;
}

float Agent::ItoR(FakeNewsModel& model) {
    const auto [N, E, I, D, R] = getNghsCounts(model);

    return computeBase(model) - E / N - I / N + D / N + R / N - S / N;
}

float Agent::DtoE(FakeNewsModel& model) {
    const auto [N, E, I, D, R] = getNghsCounts(model);

    return computeBase(model) + E / N + I / N - D / N - R / N + S / N;
}

void Agent::trimConnections(FakeNewsModel& model) {
    const auto nghs = model.network.getNeighborhood(*this);
    for (const auto ngh: nghs) {
        if (rand<0.f, 1.f>() < probOfTrim && ngh->agent.state != state) {
            model.network.removeEdge(*this, *ngh);
        }
    }
}

void Agent::addConnections(FakeNewsModel& model) {
    const auto nghs = model.network.getNeighborhood(*this, 2);

    for (const auto ngh: nghs) {
        if (rand<0.f, 1.f>() < probOfConnect && ngh->agent.state == state) {
            model.network.addEdge(*this, *ngh);
        }
    }
}

template<float From, float To>
float Agent::rand() {
    static std::random_device rd;
    static std::mt19937 gen(rd());
    static std::uniform_real_distribution dis(From, To);
    return dis(gen);
}
