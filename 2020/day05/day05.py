#!/usr/bin/env python3

with open("input.txt") as raw:
    data = raw.read()

boardings = [str(x) for x in data.split() if x]

def getRow(bp: str) -> int:
    low = 0
    up = 127
    for i in range(6):
        half = (low + up) // 2
        if bp[i] == 'F':
            up = half
        elif bp[i] == 'B':
            low = half + 1
    if bp[6] == "F":
        return low
    else:
        return up

def getCol(bp: str) -> int:
    up = 7
    low = 0
    for i in range(3):
        half = (low + up) // 2
        if bp[i]  == 'L':
            up = half
        elif bp[i] == 'R':
            low = half + 1
    if bp[2] == 'L':
        return low
    else:
        return up

def getSeatId(row: int, column: int) -> int:
    return row * 8 + column

seat_ids = []
for b in boardings:
    row = getRow(b[:7])
    col = getCol(b[-3:])
    seat_ids.append(getSeatId(row, col))

def p1():
    return(max(seat_ids))

def p2():
    for id in seat_ids:
        if id + 1 not in seat_ids and id + 2 in seat_ids:
            return id + 1

print(f"p1: {p1()}")
print(f"p2: {p2()}")
