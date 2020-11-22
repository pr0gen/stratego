# Étude du papier [Design Agents for Stratego Game](https://drive.google.com/file/d/1Y9qUYfecNeX5XotHZ0xPN51sK_4Ss9ON/view)


## Introduction 

Écrit par Sergiu Redeca et Adrian Groza, ce papier explore les designs d'agents
pour les jeux à informations partielles ou incomplète, comme le Stratego. Il
compile aussi beaucoup d'informations sur le jeu en lui même. Ils ont développé
deux agents qui utilisent les probabilités et prédictions avec (multiple-ply)
multi plis. 

Ils proposent aussi plusieurs fonctions d'évaluations pour le jeu du Stratego à
partir d'une position donnée ainsi que l'importance d'un bon placement des
pions au départ. Le jeu du Stratego se démarque, puisque contrairement aux
autres jeu de plateau, comme les échecs ou ou le Go, il a un arbre de jeu de
complexité beaucoup plus important. 

## Hypothèse(s)

- Utilisation de réseau de neuronnes, avec des algorithmes de parcours (Monte Carlo, Alpha-Béta)

- Utilisation de réseau Bayésiens et de probabilités

- Importance de l'information manquante dans le jeu.

- Importance du l'initilisation de la partie via le placements des pièces.

## Développement

### Travaux relatifs 

L'entreprise Accolade a créé le premier agent du Stratego en 1990. En
utilisant des réseaux de neuronnes, sur un plateau de jeu de 6 x 6 avec 12
pièces pour augmenter le nombre d'exécution de games par l'agent. En
considérant une place aléatoire sur le plateau avec information complète, et en
utilisant un Monte Carlo entre 10 et 50 tours de jeu, l'agent est moins fort
qu'un joueur humain, mais bats l'agent aléatoire dans 96% des cas. Des
résultats à peine meilleur, en opposant Alpha-Béta et Monte Carlos, AB gagne
dans 60% des cas.

Grâce à l'utilisation d'algorithmes stochastiques et de réseaux Bayésiens, ils
ont essayé de déterminier l'emplacement des pièces ennemies, basé sur un
apprentissage des parties précédentes contre le même joueur.  L'agent présume
par exemple, que si un joueur avance une pièce masquée A vers une pièce
démasquée B, on peut estimer que A est une pièce de rang supérieur à la pièce B
(afin de capturer). L'IA est également capable de "bluffer" en avançant une
pièce plus petite dans le but de capturer.  En modélisant, l'adversaire, dans
40% des cas l'agent effectue une bonne prediction, contrairement à 17,7% dans
le cas contraire. Ces prédictions permettent à l'agent actuel de pouvoir gagner
dans 55% des cas sans connaître le modèle de l'opposant.

Maarten et al. prouve que la recherche quiescence est efficace pour le Stratego.

	La recherche quiescente [(Quiescence search)](https://en.wikipedia.org/wiki/Quiescence_search) 
	vise à immiter l'intuition humaine dans les jeux de plateau. Par exemple,
	l'Homme sait s'il doit abandonner une partie, ou un mouvement prometteur dans
	quelques coups. 

En effet, il y a plusieurs positions de jeu sans mouvement, qui peuvent ammener
à un changement de l'état du jeu complet. Boer et Tunru ont testé la
faisabilité d'algorithmes génétiques. Le poids considère nombre de mouvements,
nombre de pièces restantes, résultat de la partie.

	Les algorithmes génétiques essayent d'approche un problème d'optimisation
	afin de résoudre celui-ci dans un temps raisonnable. Ils prennent en compte
	la selection naturelle. Ils effectuent des bonds dans les solutions à
	l'instar d'un procédure de [séparation et évaluation (branch & bound)](https://fr.wikipedia.org/wiki/S%C3%A9paration_et_%C3%A9valuation)

### Désign d'agent pour le Stratego

#### Agent probalistique

L'agent base son raisonnement sur l'issue de la partie, tous les déplacements
légaux possibles (alliés et ennemies) et toutes les informations disponibles
(rang des pièces, placements de celles-ci). Il choisit la meilleure position selon une
fonction d'évaluation. Si le rang d'une pièce est inconnu, alors l'agent
attribue avec une probabilité une valeur à cette pièce. On stocke dans une
table toutes ces informations afin de les comparer. L'agent surestime toujours
un peu les valeurs des pièces plus proche de lui, afin de ne pas se laisser
trop surprendre. En se référant à cette table, l'agent en tentant d'attaque une
pièce inconnue utilise les informations relatives à ses prédictions. Il faut
bien faire attention à la possibilité d'une pièce de pouvoir se mouvoir. 

#### Agent multi-plis
L'agent multi-plis agit de la même manière que l'agent probalistique, mais lors
d'attaque, il évalue non pas la position après l'attaque, mais il simule le
mouvement adverse. Remarque, pour simuler le mouvement adverse, il prend en
compte si l'agent est probalistique, ou multi-plis. La complexité de la tâche
est de réussir à prévoir le mouvement adversaire, à partir des informations
connues de l'agent **et** de prévoir comment l'adversaire voit notre plateau de
jeu. Il faut inverser la situation, pour baser la prédiction inverse. (Ça fait
beaucoup de guess tout ça).

![Comment on peut voir notre agent](/images/MultiPlisAgent.png)

On effectue donc un arbre de prédictions de déplacements adverses et alliés
afin de baser notre choix.

#### Phase d'initialisation

Choisir des positionnements complètement est contrairement à la pensée
jusqu'ici, plutôt une bonne idée. En effet, l'aléatoire st difficile à prévoir.
Cela dit, le fait d'avoir un placement aléatoire, limite les stratégies
offensives. En effet, nos plus hauts rangs, peuvent se retrouver en dernière
ligne. Il devient difficile de capturer. Les parties longues, sont donc compromises.

Il convient donc, de placer certaines pièces en fonctions de notre stratégie.
Par exemple, en fonction de notre adversaire.

La fonction d'évaluation ici, output un nombre négatif si le bleu gagne,
positif si rouge gagne. Elle se base sur l'avantage matériel des joueurs.
On obtient une complexité polynomiale avec O(n.m) avec n nombre de lignes et m
de colonnes. Ainsi que la distance du Drapeau ennemie. 

De manière générale, on essaye de gagner en capturant le Drapeau plutôt qu'en
prennant toutes les pièces movibles.

La dernière partie du document décrit les résultats de cet agent avec
différents paramètres, et contre différents agents. 


## Conclusion du papier (TL;DR)

La méthode la plus efficace pour le Stratego consiste à essayer de révéler un
maximum d'informations dans le jeu. L'agent doit donc se concentrer sur la
révélation d'informations pour espérer être efficient, il faut donc constituer
une base de données importante d'apprentissage.

Le placement initiale des pièces est une phase non négligeable du jeu, et
l'agent doit en prendre compte. Il faut, au possible, capturer les pièces avec
un rang proche, afin de ne pas dévoiler ses pièces secrètes trop rapidement.

En effet, l'utilisation d'algorithmes de parcours en profondeur du type Monte
Carlos ou alpha-béta, même pour un nombre important de "branches" (10-50),
l'efficacité de l'IA est moindre sans informations.

En abordant une autre méthode, en essayant de révéler de l'informations au
maximum, en prédisant la place d'une pièce le jeu semble plus facile à
résoudre. 

Peut on imaginer une combinaison des deux méthodes avec une phase
d'apprentissage contre le joueur actuel puis en utilisant des arbres ? 

La réponse est oui, on peut très bien imaginer des arbres assez conséquents sur
les probabilités des emplacements des pièces et du coup que va jouer notre
ennemie.

**Note:** Les fonctions d'évalutions de ce papier sont intéressantes. En effet,
elle se base sur l'avantage matériel pour gagner.

## Notre implémentation

Work in progress 





