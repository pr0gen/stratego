import pandas as pd
import matplotlib.pyplot as plt
import sys

def run_game_series(filename) :
    csv = pd.read_csv(filename)
    list = []
    for row in csv.iterrows():
        list.append([row[1][0],row[1][1],row[1][2]])
    return list;

def plot_victories(filename, save_images=False) :
    wins = run_game_series(filename)

    x_wins = ['Red','Blue']
    x_scores = list(range(len(wins)))

    y_wins = []
    y_scores_red_Redwins = []
    y_scores_blue_Redwins = []
    y_scores_red_Bluewins = []
    y_scores_blue_Bluewins = []
    cpt_red = 0
    cpt_blue = 0

    for i in range(len(wins)):
        y_wins.append(wins[i][0])

        if wins[i][0] == 'Red' :
            cpt_red = cpt_red + 1
            y_scores_red_Redwins.append(wins[i][1])
            y_scores_blue_Redwins.append(wins[i][2])
        if wins[i][0] == 'Blue' :
            cpt_blue = cpt_blue + 1
            y_scores_red_Bluewins.append(wins[i][1])
            y_scores_blue_Bluewins.append(wins[i][2])

    x_scores_red = list(range(cpt_red))
    x_scores_blue = list(range(cpt_blue))
    y_wins = [y_wins.count('Red'),y_wins.count('Blue')]

    width = 0.5

    fig, axs = plt.subplots(2, 1, constrained_layout=True)
    axs[0].plot(x_scores_red,y_scores_red_Redwins,width,color='r', label="joueur rouge")
    axs[0].plot(x_scores_red,y_scores_blue_Redwins,width,color='b', label="joueur bleu")
    axs[0].set_title('Scores when Red wins')
    axs[0].set_xlabel('Game')
    axs[0].set_ylabel('Score')

    fig.suptitle('Scores when each player wins', fontsize=16)

    axs[1].plot(x_scores_blue,y_scores_red_Bluewins,width,color='r', label="joueur rouge")
    axs[1].plot(x_scores_blue,y_scores_blue_Bluewins,width,color='b', label="joueur bleu")
    axs[1].set_xlabel('Game')
    axs[1].set_title('Scores when Blue wins')
    axs[1].set_ylabel('Score')

    plt.show()

    filenamepng_scores = str(filename)[:-4] + "_scores.png"

    plt.bar(x_wins,y_wins,width,color='g')
    plt.show()
    filenamepng_wins = str(filename)[:-4] + "_wins.png"
    if save_images:
        plt.savefig(filenamepng_scores)
        plt.savefig(filenamepng_wins)




