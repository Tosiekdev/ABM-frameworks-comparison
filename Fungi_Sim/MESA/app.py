import matplotlib
matplotlib.use('TkAgg') # or 'Qt5Agg'
from matplotlib import pyplot as plt
from src.model import FungiModel


model = FungiModel(width=10, height=10)

num_steps = 50
fungi_counts = []
alive_fungi_count = []
agent_res_count = []
grid_res_cells_count = []

for step in range(num_steps):
    model.step()
    num_fungi, alive_fungi, agent_res, grid_res_cells = model.get_stats()
    fungi_counts.append(num_fungi)
    alive_fungi_count.append(alive_fungi)
    agent_res_count.append(agent_res)
    grid_res_cells_count.append(grid_res_cells)
    print(f"Step {step+1}: Fungi={num_fungi}, Alive={alive_fungi}, AgentRes={agent_res:.2f}, ActiveResourceCells={grid_res_cells}")


plt.plot(fungi_counts, label="Fungi Count")
plt.plot(alive_fungi_count, label="Alive Fungi Count")
plt.plot(agent_res_count, label="Agent Resource Amount")
plt.plot(grid_res_cells_count, label="Grid Resource Cells Count")
plt.xlabel("Step")
plt.ylabel("Count / Resource")
plt.title("Fungi Simulation Over Time")
plt.grid()
plt.legend()
plt.show()
