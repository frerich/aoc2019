from collections import defaultdict
from math import ceil


def parse_substance(s):
    amount, chemical = s.split()
    return chemical, int(amount)


def parse_reaction(reaction):
    source, result  = reaction.split(" => ")
    output, num_units = parse_substance(result)
    ingredients = dict(parse_substance(s) for s in source.split(", "))
    return (output, (num_units, ingredients))


def parse(s):
    return dict(parse_reaction(line) for line in s.splitlines())


def reduce(chemical, amount, surplus):
    num_units, ingredients = reactions[chemical]
    multiples = ceil(amount / num_units)

    surplus[chemical] += multiples * num_units - amount

    components_needed = {}
    for chemical, amount in ingredients.items():
        total_amount_needed = amount * multiples
        usable_surplus = min(total_amount_needed, surplus[chemical])
        total_amount_needed -= usable_surplus
        surplus[chemical] -= usable_surplus
        components_needed[chemical] = total_amount_needed

    return components_needed


def ore_needed(amount_of_fuel):
    substances_needed = defaultdict(int)
    substances_needed['FUEL'] = amount_of_fuel

    surplus = defaultdict(int)

    ore = 0
    while substances_needed:
        chemical, amount = substances_needed.popitem()

        components_needed = reduce(chemical, amount, surplus)
        try:
            ore += components_needed.pop('ORE')
        except KeyError:
            pass

        for chemical, amount in components_needed.items():
            substances_needed[chemical] += amount

    return ore


def bsearch(lo, hi, fn):
    while hi - lo > 1:
        val = lo + (hi - lo) // 2
        if fn(val) == -1:
            hi = val
        else:
            lo = val
    return lo



with open("input.txt") as f:
    reactions = parse(f.read())

print("Part one:", ore_needed(1))
print("Part two:", bsearch(0, 1e12, lambda fuel: -1 if ore_needed(fuel) > 1e12 else 1))

