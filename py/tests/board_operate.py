import random

import numpy as np

from board.board import Board, FORMAL_CUTS
from board.fillone_minimal import compress_actions


def test(h: int, w: int):
    start = Board.random_board(h, w)
    end = start.clone()

    for cnt in range(1000):
        cut = random.choice(FORMAL_CUTS)
        to = random.randint(0, 4)
        cut_h, cut_w = cut.shape
        x, y = np.random.randint(-cut_h + 1, w), np.random.randint(
            -cut_w + 1, h
        )

        if to == 0:
            end = end.op_up(cut, x, y)
        elif to == 1:
            end = end.op_down(cut, x, y)
        elif to == 2:
            end = end.op_left(cut, x, y)
        elif to == 3:
            end = end.op_right(cut, x, y)

    _, actions = start.fillone(end)

    actions = compress_actions(h, w, actions)
    start = start.operate(actions)

    assert start == end
    print(f"Test passed: {h=}, {w=}")


if __name__ == "__main__":
    test(64, 1)
    test(1, 64)
    test(64, 64)
    test(256, 256)
    test(256, 128)
    test(128, 256)
