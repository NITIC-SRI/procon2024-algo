# dir: 0 = fill, 1 = vertical, 2 = horizontal
def gen_const_mask(size: int, dir: int) -> list[list[int]]:
    if size not in [2**i for i in range(1, 8)]:
        raise ValueError("size must be 1 or an even number")

    mask = [[0] * size for _ in range(size)]

    if dir == 0:
        for i in range(size):
            for j in range(size):
                mask[i][j] = 1
    elif dir == 1:
        for y in range(size):
            for x in range(size):
                if x % 2 == 0:
                    mask[y][x] = 1
    elif dir == 2:
        for y in range(size):
            for x in range(size):
                if y % 2 == 0:
                    mask[y][x] = 1
    return mask


if __name__ == "__main__":
    import pprint

    pprint.pprint(gen_const_mask(8, 0))
    pprint.pprint(gen_const_mask(8, 1))
    pprint.pprint(gen_const_mask(8, 2))
