#!/usr/bin/env python3

with open("input.txt") as raw_data:
    data = raw_data.read()

arr = [str(x) for x in data.split('\n') if x]

def p1():
    valid = 0
    for p in arr:
        x = int(p.split('-')[0])                                     # min
        y = int(''.join(i for i in p.split("-")[1] if i.isdigit()))  # max
        l = p.split(' ')[1].strip(':')                               # letter
        w = p.split(' ')[2]                                          # password

        if (w.count(l) < x or w.count(l) > y):
            continue
        else:
            valid += 1
    return valid

def p2():
    valid = 0
    for p in arr:
        x = int(p.split('-')[0])                                         # min
        y = int(''.join(i for i in p.split("-")[1] if i.isdigit()))      # max
        l = p.split(' ')[1].strip(':')                                   # letter
        w = p.split(' ')[2]                                              # password

        if (w[x-1] == l and w[y-1] == l):
            continue
        elif (w[x-1] == l or w[y-1] == l):
            valid += 1
        else:
            continue
    return valid

print(f"p1: {p1()}")
print(f"p2: {p2()}")
