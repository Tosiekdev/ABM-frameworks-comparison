import sys
from src.model import ConwaysGameOfLife


def main():
    if len(sys.argv) != 4:
        print("Usage: python app.py <steps> <scenario_file> <grid_size>")
        sys.exit(1)

    steps = int(sys.argv[1])
    scenario = sys.argv[2]
    size = int(sys.argv[3])

    ConwaysGameOfLife(
        scenario=scenario,
        steps=steps,
        seed=42,
        width=size,
        height=size
    )


if __name__ == "__main__":
    main()
