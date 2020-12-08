#!/usr/bin/env python3
import re
from collections import defaultdict

data = [ l.strip() for l in open('input.txt', 'r').readlines() ]

bags = defaultdict(dict)
for bag in data:
    parent_color = re.match(r'(.*) bags contain', bag).groups()[0]
    for count, b in re.findall(r'(\d+) (\w+ \w+)', bag):
        bags[parent_color][b] = int(count)

def count_bags(color: str) -> int:
    # parent to child recursion goodness
    answer = set()

    def search(color: str):
        for bag in bags:
            if color in bags[bag]:
                answer.add(bag)
                search(bag)

    search(color)
    return len(answer)

def count_child_to_parent(color: str) -> int:
    # child to parent
    def search(color: str):
        count = 1
        for bag in bags[color]:
            multiplier = bags[color][bag]
            count += multiplier * search(bag)
        return count
    return search(color) - 1

def p1():
    return count_bags('shiny gold')

def p2():
    return count_child_to_parent('shiny gold')

print(f"p1: {p1()}")
print(f"p2: {p2()}")
