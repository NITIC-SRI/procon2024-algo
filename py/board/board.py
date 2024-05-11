import copy

import numpy as np


class Board:
    def __init__(self, board: np.ndarray[int, 2]) -> None:
        self.board = board
        self.height = len(board)
        self.width = len(board[0])

    def op_right(
        self, cut: np.ndarray[int, 2], x: int, y: int
    ) -> np.ndarray[int, 2]:
        height = len(cut)
        width = len(cut[0])
        new_board = copy.deepcopy(self.board)
        for h in range(y, y + height):
            s = set()
            lst = list()
            for w in range(x, x + width):
                if cut[h - y][w - x] == 1:
                    lst.append(self.board[h][w])
                    s.add(w)
            for w in range(self.width):
                if w not in s:
                    lst.append(self.board[h][w])
            new_board[h] = lst

        return new_board

    def op_left(
        self, cut: np.ndarray[int, 2], x: int, y: int
    ) -> np.ndarray[int, 2]:
        height = len(cut)
        width = len(cut[0])
        new_board = copy.deepcopy(self.board)
        for h in range(y, y + height):
            s = set()
            lst = list()
            for w in range(x + width - 1, x - 1, -1):
                if cut[h - y][w - x] == 1:
                    lst.append(self.board[h][w])
                    s.add(w)
            for w in range(self.width - 1, -1, -1):
                if w not in s:
                    lst.append(self.board[h][w])
            new_board[h] = list(reversed(lst))
        return new_board

    def op_down(
        self, cut: np.ndarray[int, 2], x: int, y: int
    ) -> np.ndarray[int, 2]:
        height = len(cut)
        width = len(cut[0])
        new_board = copy.deepcopy(self.board)

        for w in range(x, x + width):
            s = set()
            lst = list()
            for h in range(y, y + height):
                if cut[h - y][w - x] == 1:
                    lst.append(self.board[h][w])
                    s.add(h)
            for h in range(self.height):
                if h not in s:
                    lst.append(self.board[h][w])

            for h in range(self.height):
                new_board[h][w] = lst[h]

        return new_board

    def op_up(
        self, cut: np.ndarray[int, 2], x: int, y: int
    ) -> np.ndarray[int, 2]:
        height = len(cut)
        width = len(cut[0])
        new_board = copy.deepcopy(self.board)

        for w in range(x, x + width):
            s = set()
            lst = list()
            for h in range(y + height - 1, y - 1, -1):
                if cut[h - y][w - x] == 1:
                    lst.append(self.board[h][w])
                    s.add(h)
            for h in range(self.height - 1, -1, -1):
                if h not in s:
                    lst.append(self.board[h][w])

            lst = list(reversed(lst))
            for h in range(self.height):
                new_board[h][w] = lst[h]

        return new_board
