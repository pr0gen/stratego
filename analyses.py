import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

def run_game_series() :
    csv = pd.read_csv("test_plot.csv")
    list = []
    for row in csv.iterrows():
        color = row[1][0]
        list.append(color)
    return list;


wins = run_game_series()
fig = plt.figure()
x = ['Red','Blue']
y = [wins.count('Blue'),wins.count('Red')]
width = 0.5

plt.bar(x,y,width,color='b')
plt.savefig('SimpleBar.png')
plt.show()