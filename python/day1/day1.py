def refills(mass):
    fuel = mass // 3 - 2
    if fuel > 0:
        yield fuel
        yield from refills(fuel)


with open("input.txt") as f:
    fuel = [list(refills(int(line))) for line in f]

part_one = sum(f[0] for f in fuel)
part_two = sum(sum(f) for f in fuel)

print(part_one)
print(part_two)
