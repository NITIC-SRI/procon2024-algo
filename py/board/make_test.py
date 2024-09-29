import random

import numpy as np

import board
import fillone_minimal

if __name__ == "__main__":
    h, w = 64, 1
    start = board.Board.random_board(h, w)
    end = start.clone()

    for cnt in range(1000):
        cut = np.array([[1]])
        to = random.randint(0, 4)
        x, y = np.random.randint(0, w), np.random.randint(0, h)

        if to == 0:
            end = end.op_up(cut, x, y)
        elif to == 1:
            end = end.op_down(cut, x, y)
        elif to == 2:
            end = end.op_left(cut, x, y)
        elif to == 3:
            end = end.op_right(cut, x, y)

    _, actions = start.fillone(end)

    actions = fillone_minimal.compress_actions(h, w, actions)

    sboard = start.board.tolist()
    eboard = end.board.tolist()

    fstr1 = "vec!{},"

    fstr = "Board::new(vec![{}])"
    sb_str = fstr.format(" ".join([fstr1.format(l) for l in sboard]))
    eb_str = fstr.format(" ".join([fstr1.format(l) for l in eboard]))

    print(f"({sb_str}, {eb_str}, {len(actions)}), ")
    # print("start")
    # print(start.board)

    # print("end")
    # print(end.board)

    # print(len(actions))
