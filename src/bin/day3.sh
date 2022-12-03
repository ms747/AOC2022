#!/bin/bash
CHARS="$(echo {a..z} | tr -d ' ')$(echo {A..Z} | tr -d ' ')"
SUM1=0
SUM2=0

part1 () {
	mid=$(($(echo $1 | wc -c) / 2))
	char=$(comm -12 <(fold -w1 <<< ${1:0:mid} | sort -u) <(fold -w1 <<< ${1:mid} | sort -u))
	prefix=${CHARS%%$char*}
	((SUM1+=$((${#prefix} + 1))))
}

part2 () {
	char=$(comm -12 <(fold -w1 <<< ${3} | sort -u) <(comm -12 <(fold -w1 <<< ${1} | sort -u) <(fold -w1 <<< ${2} | sort -u)))
	prefix=${CHARS%%$char*}
	((SUM2+=$((${#prefix} + 1))))
}

while read -r line
do 
	part1 $line 
done < $1

while read line1; read line2; read line3
do
	part2 $line1 $line2 $line3
done < $1

echo "Part1: $SUM1"
echo "Part2: $SUM2"
