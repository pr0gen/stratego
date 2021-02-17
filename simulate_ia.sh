#!/bin/bash

read -p "Type filename (ex : test.csv) : " filename
read -p "Number of thread you wanna use (ex : 4) : " numberOfThread
read -p "Number of game you wanna simulate (ex: 50) : " numberOfGame

echo "Which AI you wanna use as player 1 : "
echo "1. Random"
echo "2. Monte_carlo"
echo "3. Quit"
echo -n "Type 1 or 2 or 3 :"
read -n 1 -t 15 a
printf "\n"
case $a in
1* )     first_ai_name="random";;

2* )     first_ai_name="monte_carlo";;

3* )     exit 0;;

* )     echo "Try again.";;
esac


echo "Which AI you wanna use as player 2 : "
echo "1. Random"
echo "2. Monte_carlo"
echo "3. Quit"
echo -n "Type 1 or 2 or 3 :"
read -n 1 -t 15 a
printf "\n"
case $a in
1* )     second_ai_name="random";;

2* )     second_ai_name="monte_carlo";;

3* )     exit 0;;

* )     echo "Try again.";;
esac

./analyses $filename $numberOfThread $numberOfGame $first_ai_name $second_ai_name
python3 analyses.py $filename