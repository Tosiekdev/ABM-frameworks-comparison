from src.model import FakeNewsModel
import matplotlib.pyplot as plt

model = FakeNewsModel()

NUM_STEPS = 100

for _ in range(NUM_STEPS):
    model.step()

plt.plot(model.datacollector.get_model_vars_dataframe())
plt.show()