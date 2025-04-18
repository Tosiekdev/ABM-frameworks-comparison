from enum import Enum


class State(Enum):
    SUSCEPTIBLE = "S"
    EXPOSED = "E"
    INFECTED = "I"
    DOUBTFUL = "D"
    RECOVERED = "R"


class AgeGroup(Enum):
    from00to09 = 1
    from10to19 = 2
    from20to29 = 3
    from30to39 = 4
    from40to49 = 5
    from50to59 = 6
    from60to69 = 7
    from70to79 = 8
    from80toXX = 9


class EducationGroup(Enum):
    PRIMARY = 1
    SECONDARY = 2
    HIGHER = 3
    VOCATIONAL = 4


class SexGroup(Enum):
    MALE = 1
    FEMALE = 2
    OTHER = 3


EducationDistribution = {
    State.SUSCEPTIBLE: {
        EducationGroup.PRIMARY: 0.02,
        EducationGroup.SECONDARY: 0.18,
        EducationGroup.HIGHER: 0.75,
        EducationGroup.VOCATIONAL: 0.05,
    },
    State.EXPOSED: {
        EducationGroup.PRIMARY: 0.10,
        EducationGroup.SECONDARY: 0.40,
        EducationGroup.HIGHER: 0.45,
        EducationGroup.VOCATIONAL: 0.05,
    },
    State.INFECTED: {
        EducationGroup.PRIMARY: 0.05,
        EducationGroup.SECONDARY: 0.30,
        EducationGroup.HIGHER: 0.60,
        EducationGroup.VOCATIONAL: 0.05,
    },
    State.DOUBTFUL: {
        EducationGroup.PRIMARY: 0.03,
        EducationGroup.SECONDARY: 0.27,
        EducationGroup.HIGHER: 0.65,
        EducationGroup.VOCATIONAL: 0.05,
    },
    State.RECOVERED: {
        EducationGroup.PRIMARY: 0.04,
        EducationGroup.SECONDARY: 0.35,
        EducationGroup.HIGHER: 0.55,
        EducationGroup.VOCATIONAL: 0.06,
    }
}

SexDistribution = {
    State.SUSCEPTIBLE: {
        SexGroup.MALE: 0.55,
        SexGroup.FEMALE: 0.40,
        SexGroup.OTHER: 0.05,
    },
    State.EXPOSED: {
        SexGroup.MALE: 0.48,
        SexGroup.FEMALE: 0.48,
        SexGroup.OTHER: 0.04,
    },
    State.INFECTED: {
        SexGroup.MALE: 0.45,
        SexGroup.FEMALE: 0.50,
        SexGroup.OTHER: 0.05,
    },
    State.DOUBTFUL: {
        SexGroup.MALE: 0.50,
        SexGroup.FEMALE: 0.45,
        SexGroup.OTHER: 0.05,
    },
    State.RECOVERED: {
        SexGroup.MALE: 0.52,
        SexGroup.FEMALE: 0.43,
        SexGroup.OTHER: 0.05,
    }
}

PlatformAgeDistribution = {
    State.SUSCEPTIBLE: {
        AgeGroup.from00to09: 0.00,
        AgeGroup.from10to19: 0.05,
        AgeGroup.from20to29: 0.40,
        AgeGroup.from30to39: 0.30,
        AgeGroup.from40to49: 0.15,
        AgeGroup.from50to59: 0.07,
        AgeGroup.from60to69: 0.03,
        AgeGroup.from70to79: 0.00,
        AgeGroup.from80toXX: 0.00,
    },
    State.EXPOSED: {
        AgeGroup.from00to09: 0.02,
        AgeGroup.from10to19: 0.10,
        AgeGroup.from20to29: 0.30,
        AgeGroup.from30to39: 0.25,
        AgeGroup.from40to49: 0.15,
        AgeGroup.from50to59: 0.10,
        AgeGroup.from60to69: 0.05,
        AgeGroup.from70to79: 0.02,
        AgeGroup.from80toXX: 0.01,
    },
    State.INFECTED: {
        AgeGroup.from00to09: 0.01,
        AgeGroup.from10to19: 0.26,
        AgeGroup.from20to29: 0.41,
        AgeGroup.from30to39: 0.20,
        AgeGroup.from40to49: 0.07,
        AgeGroup.from50to59: 0.03,
        AgeGroup.from60to69: 0.01,
        AgeGroup.from70to79: 0.01,
        AgeGroup.from80toXX: 0.00,
    },
    State.DOUBTFUL: {
        AgeGroup.from00to09: 0.00,
        AgeGroup.from10to19: 0.05,
        AgeGroup.from20to29: 0.35,
        AgeGroup.from30to39: 0.35,
        AgeGroup.from40to49: 0.15,
        AgeGroup.from50to59: 0.05,
        AgeGroup.from60to69: 0.03,
        AgeGroup.from70to79: 0.02,
        AgeGroup.from80toXX: 0.00,
    },
    State.RECOVERED: {
        AgeGroup.from00to09: 0.00,
        AgeGroup.from10to19: 0.10,
        AgeGroup.from20to29: 0.35,
        AgeGroup.from30to39: 0.30,
        AgeGroup.from40to49: 0.15,
        AgeGroup.from50to59: 0.07,
        AgeGroup.from60to69: 0.02,
        AgeGroup.from70to79: 0.01,
        AgeGroup.from80toXX: 0.00,
    }
}
