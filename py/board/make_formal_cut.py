import pickle


def make_type1_cut(size):
    return [[1] * size for _ in range(size)]


def make_type2_cut(size):
    return [[1] * size if i % 2 == 0 else [0] * size for i in range(size)]


def make_type3_cut(size):
    tmp = make_type2_cut(size)
    tmp2 = [[0] * size for _ in range(size)]

    for i in range(size):
        for j in range(size):
            tmp2[i][j] = tmp[j][i]

    return tmp2


if __name__ == "__main__":
    N = 9

    cuts_type1 = []
    cuts_type2 = []
    cuts_type3 = []

    # size 1

    # cuts_type1.append([[1]])

    for i in range(N):
        cuts_type1.append(make_type1_cut(2**i))
        cuts_type2.append(make_type2_cut(2**i))
        cuts_type3.append(make_type3_cut(2**i))

    formal_cuts = [cuts_type1[0]]

    for i in range(1, N):
        formal_cuts.append(cuts_type1[i])
        formal_cuts.append(cuts_type2[i])
        formal_cuts.append(cuts_type3[i])

    with open("./board/formal_cut.pickle", "wb") as f:
        pickle.dump(formal_cut, f)
