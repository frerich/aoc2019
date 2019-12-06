from collections import defaultdict


def part_one(orbital_map):
    satellites = defaultdict(list)
    for object, satellite in orbital_map:
        satellites[object].append(satellite)

    distance_from_com = {"COM": 0}

    queue = ["COM"]
    while queue:
        obj = queue.pop()
        for sat in satellites[obj]:
            distance_from_com[sat] = distance_from_com[obj] + 1
            queue.append(sat)

    return sum(distance_from_com.values())


def part_two(orbital_map):
    object_by_satellite = {satellite: object for object, satellite in orbital_map}

    def route_to_com(obj):
        while obj != "COM":
            obj = object_by_satellite[obj]
            yield obj

    my_route = {obj: num_transfers for num_transfers, obj in enumerate(route_to_com("YOU"))}
    for num_transfers, obj in enumerate(route_to_com("SAN")):
        if obj in my_route:
            return my_route[obj] + num_transfers


with open("input.txt") as f:
    orbital_map = [line.strip().split(")") for line in f]

print("Part one: %d" % part_one(orbital_map))
print("Part two: %d" % part_two(orbital_map))


