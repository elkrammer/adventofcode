#!/usr/bin/python3

import re
from collections import defaultdict

lines = [x.strip() for x in open('input.txt', 'r').readlines()]

bags = defaultdict(dict)
for l in lines:
    bag = re.match(r'(.*) bags contain', l).groups()[0]
    for count, b in re.findall(r'(\d+) (\w+ \w+) bag', l):
        bags[bag][b] = int(count)

def part1():
    answer = set()
    def search(color):
        for b in bags:
            if color in bags[b]:
                answer.add(b)
                search(b)
    search('shiny gold')
    print(answer)
    return len(answer)

print(part1())
