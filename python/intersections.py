import timeit

O = (0, 0)

directions = {
    'R': (1, 0),
    'L': (-1, 0),
    'U': (0, 1),
    'D': (0, -1)
}


def addPos(a, b):
    return (a[0] + b[0], a[1] + b[1])


def tracePath(path):
    encountered = set()
    ops = path.split(",")
    pos = O
    for op in ops:
        d = directions[op[0]]
        amount = int(op[1:])
        while amount > 0:
            pos = addPos(pos, d)
            encountered.add(pos)
            amount = amount - 1
    return encountered


def answer(intersections):
    ans = []
    for i in intersections:
        ans.append((abs(i[0]) + abs(i[1]), i))
    ans = sorted(ans)
    if len(ans) == 0:
        print("no intersections")
    else:
        print(ans[0])


def main():

    line = input()

    start = timeit.default_timer()
    firstPath, secondPath = line.split(";")

    firstSet = tracePath(firstPath)
    secondSet = tracePath(secondPath)

    intersection = firstSet.intersection(secondSet)

    answer(intersection)
    end = timeit.default_timer()
    print(f"took {end-start}")


if __name__ == "__main__":
    main()
