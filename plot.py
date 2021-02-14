import pandas as pd

def run_game_series() :
    csv = pd.read_csv("test_plot.csv")
    blue = 0
    red = 0
    count = 0
    for row in csv.iterrows():
        color = row[1][0]
        if color == "Blue" :
            blue += 1
        if color == "Red" :
            red += 1
    return blue, red, count;

print(run_game_series())