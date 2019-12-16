from itertools import cycle, repeat, chain


def pattern(idx):
    iterable = cycle(chain(*(repeat(d, idx + 1) for d in [0, 1, 0, -1])))
    next(iterable)
    return iterable


def phase(signal):
    result = []
    for i in range(len(signal)):
        products = [d * p for d, p in zip(signal, pattern(i))]
        result.append(abs(sum(products)) % 10)
    return result


def decode(signal, num_phases, offset):
    signal = list(map(int, signal))
    for i in range(num_phases):
        signal = phase(signal)
    return ''.join(map(str, signal[offset:offset + 8]))


def part_one(signal):
    return decode(signal, 100, 0)


def part_two(signal):
    return decode(signal * 10000, 100, int(signal[:7]))


with open("input.txt") as f:
    signal = f.read().rstrip()

print("Part one:", part_one(signal))
