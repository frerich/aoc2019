WIDTH = 25
HEIGHT = 6


def chunks(xs, size):
    return [xs[i:i + size] for i in range(0, len(xs), size)]


def part_one(layers):
    least_zeroes = min(layers, key=lambda layer: layer.count(0))
    return least_zeroes.count(1) * least_zeroes.count(2)


def part_two(layers):
    merged = (next(n for n in stack if n != 2) for stack in zip(*layers))
    rendered = ''.join(' ' if n == 0 else '#' for n in merged)
    return '\n'.join(chunks(rendered, WIDTH))


with open("input.txt") as f:
    digits = [int(c) for c in f.read().rstrip()]
    layers = chunks(digits, WIDTH * HEIGHT)

print("Part one: %d" % part_one(layers))
print("Part two:\n%s" % part_two(layers))
