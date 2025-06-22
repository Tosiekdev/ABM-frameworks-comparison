package sim.app.fake_news;

import java.util.EnumMap;
import java.util.Map;

public class DemographicData {
    public EnumMap<State, EnumMap<EducationGroup, Double>> educationDistribution = new EnumMap<>(State.class);
    public EnumMap<State, EnumMap<SexGroup, Double>> sexDistribution = new EnumMap<>(State.class);
    public EnumMap<State, EnumMap<AgeGroup, Double>> platformAgeDistribution = new EnumMap<>(State.class);

    public DemographicData() {


        educationDistribution.put(State.SUSCEPTIBLE, new EnumMap<>(Map.of(
                EducationGroup.PRIMARY, 0.02,
                EducationGroup.SECONDARY, 0.18,
                EducationGroup.HIGHER, 0.75,
                EducationGroup.VOCATIONAL, 0.05
        )));

        educationDistribution.put(State.EXPOSED, new EnumMap<>(Map.of(
                EducationGroup.PRIMARY, 0.10,
                EducationGroup.SECONDARY, 0.40,
                EducationGroup.HIGHER, 0.45,
                EducationGroup.VOCATIONAL, 0.05
        )));

        educationDistribution.put(State.INFECTED, new EnumMap<>(Map.of(
                EducationGroup.PRIMARY, 0.05,
                EducationGroup.SECONDARY, 0.30,
                EducationGroup.HIGHER, 0.60,
                EducationGroup.VOCATIONAL, 0.05
        )));

        educationDistribution.put(State.DOUBTFUL, new EnumMap<>(Map.of(
                EducationGroup.PRIMARY, 0.03,
                EducationGroup.SECONDARY, 0.27,
                EducationGroup.HIGHER, 0.65,
                EducationGroup.VOCATIONAL, 0.05
        )));

        educationDistribution.put(State.RECOVERED, new EnumMap<>(Map.of(
                EducationGroup.PRIMARY, 0.04,
                EducationGroup.SECONDARY, 0.35,
                EducationGroup.HIGHER, 0.55,
                EducationGroup.VOCATIONAL, 0.06
        )));

        sexDistribution.put(State.SUSCEPTIBLE, new EnumMap<>(Map.of(
                SexGroup.MALE, 0.55,
                SexGroup.FEMALE, 0.40,
                SexGroup.OTHER, 0.05
        )));

        sexDistribution.put(State.EXPOSED, new EnumMap<>(Map.of(
                SexGroup.MALE, 0.48,
                SexGroup.FEMALE, 0.48,
                SexGroup.OTHER, 0.04
        )));

        sexDistribution.put(State.INFECTED, new EnumMap<>(Map.of(
                SexGroup.MALE, 0.45,
                SexGroup.FEMALE, 0.50,
                SexGroup.OTHER, 0.05
        )));

        sexDistribution.put(State.DOUBTFUL, new EnumMap<>(Map.of(
                SexGroup.MALE, 0.50,
                SexGroup.FEMALE, 0.45,
                SexGroup.OTHER, 0.05
        )));

        sexDistribution.put(State.RECOVERED, new EnumMap<>(Map.of(
                SexGroup.MALE, 0.52,
                SexGroup.FEMALE, 0.43,
                SexGroup.OTHER, 0.05
        )));

        platformAgeDistribution.put(State.SUSCEPTIBLE, new EnumMap<>(Map.of(
                AgeGroup.from00to09, 0.00,
                AgeGroup.from10to19, 0.05,
                AgeGroup.from20to29, 0.40,
                AgeGroup.from30to39, 0.30,
                AgeGroup.from40to49, 0.15,
                AgeGroup.from50to59, 0.07,
                AgeGroup.from60to69, 0.03,
                AgeGroup.from70to79, 0.00,
                AgeGroup.from80toXX, 0.00
        )));

        platformAgeDistribution.put(State.EXPOSED, new EnumMap<>(Map.of(
                AgeGroup.from00to09, 0.02,
                AgeGroup.from10to19, 0.10,
                AgeGroup.from20to29, 0.30,
                AgeGroup.from30to39, 0.25,
                AgeGroup.from40to49, 0.15,
                AgeGroup.from50to59, 0.10,
                AgeGroup.from60to69, 0.05,
                AgeGroup.from70to79, 0.02,
                AgeGroup.from80toXX, 0.01
        )));

        platformAgeDistribution.put(State.INFECTED, new EnumMap<>(Map.of(
                AgeGroup.from00to09, 0.01,
                AgeGroup.from10to19, 0.26,
                AgeGroup.from20to29, 0.41,
                AgeGroup.from30to39, 0.20,
                AgeGroup.from40to49, 0.07,
                AgeGroup.from50to59, 0.03,
                AgeGroup.from60to69, 0.01,
                AgeGroup.from70to79, 0.01,
                AgeGroup.from80toXX, 0.00
        )));

        platformAgeDistribution.put(State.DOUBTFUL, new EnumMap<>(Map.of(
                AgeGroup.from00to09, 0.00,
                AgeGroup.from10to19, 0.05,
                AgeGroup.from20to29, 0.35,
                AgeGroup.from30to39, 0.35,
                AgeGroup.from40to49, 0.15,
                AgeGroup.from50to59, 0.05,
                AgeGroup.from60to69, 0.03,
                AgeGroup.from70to79, 0.02,
                AgeGroup.from80toXX, 0.00
        )));

        platformAgeDistribution.put(State.RECOVERED, new EnumMap<>(Map.of(
                AgeGroup.from00to09, 0.00,
                AgeGroup.from10to19, 0.10,
                AgeGroup.from20to29, 0.35,
                AgeGroup.from30to39, 0.30,
                AgeGroup.from40to49, 0.15,
                AgeGroup.from50to59, 0.07,
                AgeGroup.from60to69, 0.02,
                AgeGroup.from70to79, 0.01,
                AgeGroup.from80toXX, 0.00
        )));

    }
}
