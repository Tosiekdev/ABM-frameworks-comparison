#pragma once

#include <functional>

struct FakeNewsModel;

struct Agent {
    enum State {
        S, E, I, D, R
    };

    enum Education {
        Primary, Secondary, Higher, Vocational
    };

    enum Sex {
        Male, Female, Other
    };

    enum AgeGroup {
        From00to09, From10to19, From20to29, From30to39, From40to49, From50to59, From60to69, From70to79, From80toXX
    };

    explicit Agent(const State s, const Education e, const Sex sx, const AgeGroup a) : id(++idCounter), state(s),
        education(e), sex(sx), ageGroup(a) {}

    int id;

    void step(FakeNewsModel& model);

    [[nodiscard]] bool isActive() const {
        return true;
    };

    State state;
    Education education;
    Sex sex;
    AgeGroup ageGroup;

    static int idCounter;

private:
    float StoE(FakeNewsModel& model);

    float EtoI(FakeNewsModel& model);

    float EtoD(FakeNewsModel& model);

    float ItoR(FakeNewsModel& model);

    float DtoE(FakeNewsModel& model);

    void trimConnections(FakeNewsModel& model);

    void addConnections(FakeNewsModel& model);

    static constexpr float alfa = 0.1f;
    static constexpr float beta = 0.1f;
    static constexpr float gamma = 0.1f;
    static constexpr float delta = 0.1f;
    static constexpr float theta = 0.1f;

    static constexpr float probOfTrim = 0.03f;
    static constexpr float probOfConnect = 0.5f;

    template<float From, float To>
    static float rand();
};

template<>
struct std::hash<Agent> {
    size_t operator()(const Agent& agent) const noexcept {
        return std::hash<int>{}(agent.id);
    }
};
