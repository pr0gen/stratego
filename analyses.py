from ai_python.src.analyses import run_game_series,plot_victories
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys
import os

filename = "data.csv"
first_ai = "monte_carlo_v2" 
secondai = "random" 
numberOfParties = "30"

my_cmd = './analyzes ' + filename + " " + numberOfParties + " " + first_ai + " " + secondai

os.system(my_cmd)

plot_victories(filename, True)
