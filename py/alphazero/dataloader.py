from board import Board
import numpy as np


# 0 padding
def zero_board_padding(board: Board, s: int = 256) -> list:
    b = np.zeros((s, s), dtype=np.float32)
    for y in range(board.height):
        for x in range(board.width):
            b[y][x] = board.board[y][x]
    return b


# mask board
def mask_board(h, w, s: int = 256):
    b = np.zeros((s, s), dtype=np.float32)
    for y in range(h):
        for x in range(w):
            b[y][x] = 1.0
    return b


def split_layers(board: Board, s: int = 256):
    mask = mask_board(board.height, board.width, s)

    b = zero_board_padding(board, s)
    b0 = (b == 0).astype(np.float32) * mask
    b1 = (b == 1).astype(np.float32)
    b2 = (b == 2).astype(np.float32)
    b3 = (b == 3).astype(np.float32)

    return mask, b0, b1, b2, b3


if __name__ == "__main__":
    import pprint

    board = Board.random_board(8, 8)

    mask, b0, b1, b2, b3 = split_layers(board, 10)

    pprint.pprint(mask)
    pprint.pprint(b0)
    pprint.pprint(b1)
    pprint.pprint(b2)
    pprint.pprint(b3)
