import random
import time

import matplotlib.pyplot as plt
import numpy as np
import ray
from board import Board
from fillone_minimal import compress_actions

ray.init()


sample = 1
H, W = 256, 256
tests = []
for _ in range(sample):
    start = Board.random_board(H, W)
    end = start.clone()
    for cnt in range(random.randint(0, 100)):
        cut = np.array([
            [0]
        ])
        to = random.randint(0, 4)
        x, y = np.random.randint(-1 * H // 2, H // 2), np.random.randint(
            -1 * W // 2, W // 2
        )

        if to == 0:
            end = end.op_up(cut, x, y)
        elif to == 1:
            end = end.op_down(cut, x, y)
        elif to == 2:
            end = end.op_left(cut, x, y)
        elif to == 3:
            end = end.op_right(cut, x, y)

    tests.append((start, end, cnt))


@ray.remote
def make_test(sample):
    start, end, cnt = sample
    new, actions = start.fillone(end)
    assert new == end
    # print(f"Test {i} passed. action length: {len(actions)}, cnt: {cnt}")

    com = compress_actions(H, W, actions)
    print(com)
    return len(actions), cnt, len(com)


res = [make_test.remote(test) for test in tests]

begin_time = time.time()

x = []
y = []
z = []

for r in ray.get(res):
    x.append(r[0])
    y.append(r[1])
    z.append(r[2])

print(min(z))

end_time = time.time()

print(f"Time: {end_time - begin_time}")

# print(np.corrcoef(np.array([xs, ys])))
# plt.scatter(x, z)
# plt.savefig("fillone_plot.png")

plt.scatter(y, z)
plt.savefig("fillone_plot2.png")
