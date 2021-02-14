import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys


def run_game_series(filename) :
    csv = pd.read_csv(filename)
    list = []
    for row in csv.iterrows():
        color = row[1][0]
        list.append(color)
    return list;

def plot_victories(filename) :
    wins = run_game_series(filename)
    fig = plt.figure()
    x = ['Red','Blue']
    y = [wins.count('Blue'),wins.count('Red')]
    width = 0.5

    filenamepng = filename.split()
    filenamepng = filenamepng[:-4]
    filenamepng = join(filenamepng)

    plt.bar(x,y,width,color='b')
    plt.savefig(filenamepng)
    plt.show()

plot_victories(sys.argv[1])