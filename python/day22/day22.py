from functools import reduce


def parse_techniques(path, deck_size):
    with open(path) as f:
        for line in f:
            if line.startswith('cut'):
                delta = int(line.split()[1])
                yield lambda x: (x - delta) % deck_size
            elif line.startswith('deal with increment'):
                inc = int(line.split()[3])
                yield lambda x: (x * inc) % deck_size
            else:
                yield lambda x: deck_size - x - 1


def compose(f, g):
    return lambda x: f(g(x))


shuffle = reduce(compose, parse_techniques("input.txt", 10007))
print("Part one:", shuffle(2019))
