def steps(line):
    for segment in line.split(","):
        if segment[0] == 'U':
            delta = (0, -1)
        elif segment[0] == 'D':
            delta = (0, 1)
        elif segment[0] == 'L':
            delta = (-1, 0)
        elif segment[0] == 'R':
            delta = (1, 0)

        yield from [delta] * int(segment[1:])


def wire(line):
    x, y = 0, 0
    for dx, dy in steps(line):
        yield (x, y)
        x += dx
        y += dy
    yield (x, y)


def manhattan(pos):
    return abs(pos[0]) + abs(pos[1])


with open("input.txt") as f:
    wires = [list(wire(line)) for line in f]

collisions = set(wires[0][1:]) & set(wires[1][1:])

print("Part one: %d" % min(manhattan(pos) for pos in collisions))
print("Part two: %d" % min(wires[0].index(pos) + wires[1].index(pos) for pos in collisions))
