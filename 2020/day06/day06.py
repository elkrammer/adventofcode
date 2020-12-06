#!/usr/bin/env python3
with open("input.txt") as raw:
    data = raw.read().strip().split("\n\n")

groups = [set(line.replace("\n", "")) for line in data]

def p1():
    return sum([len(g) for g in groups])

def p2():
    sum = 0
    for group in data:
        group = group.split()
        first = set(group[0])
        for g in group[1:]:
            first = first.intersection(g)
        sum += len(first)
    return sum

print(f"p1: {p1()}")
print(f"p2: {p2()}")
