#!/bin/bash
# affiche_param.sh
# 1. filename
# 2. nbr of thread
# 3. nbr of game
# 4. first ai name
# 5. second ai name

echo "Le 1er paramètre est : $1"

echo "test" > $1

counter=1
until [ $counter -gt $3 ]
do

echo $counter >> $1
((counter++))

done


