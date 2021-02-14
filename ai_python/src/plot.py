import pandas as pd

def run_game_series() :
    csv = pd.read_csv("test_plot.csv")
    print(csv)
    return csv

run_game_series()