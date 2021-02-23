from ai_python.src.analyses import run_game_series,plot_victories
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys
import os

filename = "data.csv"
first_ai = "monte_carlo" 
secondai = "random" 
numberOfParties = "10"

my_cmd = './analyzes ' + filename + " " + numberOfParties + " " + first_ai + " " + secondai

os.system(my_cmd)

plot_victories(filename)
