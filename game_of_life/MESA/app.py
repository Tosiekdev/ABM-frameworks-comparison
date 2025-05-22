from game_of_life.MESA.src.model import ConwaysGameOfLife

size=10
game_of_life = ConwaysGameOfLife(
    scenario = f"../test_scenarios/{size}.txt",
    steps = 10,
    seed=42,
    width=size,
    height=size
)