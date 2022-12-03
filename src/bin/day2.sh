#!/bin/bash

part1=0
AX=4 AY=8 AZ=3 BX=1 BY=5 BZ=9 CX=7 CY=2 CZ=6
while read -r line; do ((part1+=line)); done < <(cat input.txt | sed 's/ //g')

part2=0
AX=3 AY=4 AZ=8 BX=1 BY=5 BZ=9 CX=2 CY=6 CZ=7
while read -r line; do ((part2+=line)); done < <(cat input.txt | sed 's/ //g')

echo $part1
echo $part2
