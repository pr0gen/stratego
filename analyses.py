from ai_python.src.analyses import run_game_series,plot_victories
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys
import os

filename = "data2.csv"
first_ai = "random"
secondai = "monte_carlo"
numberOfParties = "50"

my_cmd = './analyzes ' + filename + " " + numberOfParties + " " + first_ai + " " + secondai

os.system(my_cmd)

plot_victories(filename)
