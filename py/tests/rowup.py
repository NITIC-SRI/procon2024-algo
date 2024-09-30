import numpy as np

from board.board import Board

if __name__ == "__main__":
    start = Board(np.array([[1 * i] * 5 for i in range(5)]))
    start._row_up(5)
    print(start.board)
