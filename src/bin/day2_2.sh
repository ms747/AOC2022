#!/bin/bash

# Rock = A, X
# Paper = B, Y
# Scissor = C, Z

WIN=6
LOSE=0
DRAW=3

ROCK=1
PAPER=2
SCISSOR=3

while read -r a b; do
	if [[ "$a" == "A" ]]; then
		if [[ "$b" == "X" ]]; then ((sum += ROCK + DRAW)) && ((sum2 += SCISSOR + LOSE))
		elif [[ "$b" == "Y" ]]; then ((sum += PAPER + WIN)) && ((sum2 += ROCK + DRAW))
		else ((sum += SCISSOR + LOSE)) && ((sum2 += PAPER + WIN))
		fi
	elif [[ "$a" == "B" ]]; then
		if [[ "$b" == "X" ]]; then ((sum += ROCK + LOSE)) && ((sum2 += ROCK + LOSE))
		elif [[ "$b" == "Y" ]]; then ((sum += PAPER + DRAW)) && ((sum2 += PAPER + DRAW))
		else ((sum += SCISSOR + WIN)) && ((sum2 += SCISSOR + WIN))
		fi
	else
		if [[ "$b" == "X" ]]; then ((sum += ROCK + WIN)) &&((sum2 += PAPER + LOSE))
		elif [[ "$b" == "Y" ]]; then ((sum += PAPER + LOSE)) && ((sum2 += SCISSOR + DRAW))
		else ((sum += SCISSOR + DRAW)) && ((sum2 += ROCK + WIN))
		fi
	fi
done < input.txt

echo $sum
echo $sum2

