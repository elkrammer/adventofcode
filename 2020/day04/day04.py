#!/usr/bin/env python3
import re

with open("input.txt") as raw_data:
    data = raw_data.read()

passports = [str(x) for x in data.split('\n\n') if x]
required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]

def mapPassport(passport: str) -> dict:
    pp = {}
    for line in passport.split():
        k, v = line.split(':')
        pp[k] = v
    return pp

def isPassportValid(passport: dict) -> bool:
    return all(field in passport.keys() for field in required)

def isValidHeight(height: str) -> bool:
    if height.endswith("cm"):
        h = height.replace("cm", "")
        if int(h) >= 150 and int(h) <= 193:
            return True
    elif height.endswith("in"):
        h = height.replace("in", "")
        if int(h) >= 59 and int(h) <= 76:
            return True
    return False

def ruleValidation(p: dict) -> bool:
    checks = [
        1920 <= int(p.get('byr', -1)) <= 2002,
        2010 <= int(p.get('iyr', -1)) <= 2020,
        2020 <= int(p.get('eyr', -1)) <= 2030,
        isValidHeight(p.get('hgt', '')),
        p.get('ecl') in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'],
        re.match(r"^#[0-9a-f]{6}$", p.get('hcl', '')),
        re.match(r"^[0-9]{9}$", p.get('pid', '')),
    ]
    return all(checks)

def p1() -> int:
    valid = 0
    for p in passports:
        if isPassportValid(mapPassport(p)):
            valid += 1
    return valid

def p2() -> int:
    valid = 0
    for p in passports:
        pp = mapPassport(p)
        if isPassportValid(pp) and ruleValidation(pp):
            valid += 1
    return valid

print(f"p1: {p1()}")
print(f"p2: {p2()}")
