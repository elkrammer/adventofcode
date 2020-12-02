#!/usr/bin/env python3

with open("input.txt") as raw_data:
    data = raw_data.read()

arr = [int(x) for x in data.split('\n') if x]

def p1():
    for x in arr:
        for y in arr:
            if x + y == 2020:
                return x * y

def p2():
    for x in arr:
        for y in arr:
            for z in arr:
                if x + y + z == 2020:
                    return x * y * z

print(f"p1: {p1()}")
print(f"p2: {p2()}")
