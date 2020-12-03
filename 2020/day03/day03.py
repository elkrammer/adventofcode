#!/usr/bin/env python3

with open("input.txt") as raw_data:
    data = raw_data.read()

map = [str(x) for x in data.split('\n') if x]
width = len(map[0])

def findTreesInSlope(slope):
    trees = 0
    pos = [0,0]
    while pos[0] < len(map):
        if map[pos[0]][pos[1]] == "#":
             trees += 1
        pos[0] += slope[0]
        pos[1] += slope[1]
        pos[1] = pos[1] % width
    return trees

def p1():
    slope = [1,3]
    return findTreesInSlope(slope)

def p2():
    slopes = [[1,1], [1,3], [1,5], [1,7], [2,1]]
    results = 1

    for slope in slopes:
        trees = int()
        trees += findTreesInSlope(slope)
        results *= trees
    return results

print(f"p1: {p1()}")
print(f"p2: {p2()}")
