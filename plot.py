import pandas as pd
from matplotlib import pyplot as plt

# Referring to example code from https://www.tutorialspoint.com/plot-data-from-csv-file-with-matplotlib
# and
# https://www.tutorialspoint.com/make-a-multiline-plot-from-csv-file-in-matplotlib

plt.rcParams["figure.figsize"] = [7.00, 3.50]
plt.rcParams["figure.autolayout"] = True
columns = ["Amplitude"]
df = pd.read_csv("frequencies.csv", usecols=columns)
plt.plot(df.Amplitude)
plt.show()
