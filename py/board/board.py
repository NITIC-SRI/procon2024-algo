import copy
from typing import Self

import numpy as np

Direction = {"up": 0, "right": 1, "down": 2, "left": 3}


class Board:
    def __init__(self, board: np.ndarray[int, 2]) -> None:
        self._board = board
        self._height = len(board)
        self._width = len(board[0])

    @property
    def board(self) -> np.ndarray[int, 2]:
        return self._board

    @property
    def height(self) -> int:
        return self._height

    @property
    def width(self) -> int:
        return self._width

    def op_right(self, cut: np.ndarray[int, 2], x: int, y: int) -> Self:
        height = len(cut)
        width = len(cut[0])
        new_board = copy.deepcopy(self._board)
        for h in range(y, y + height):
            if not 0 <= h < self._height:
                continue
            s = set()
            lst = list()
            for w in range(x, x + width):
                if not 0 <= w < self._width:
                    continue
                if cut[h - y][w - x] == 1:
                    lst.append(self._board[h][w])
                    s.add(w)
            for w in range(self._width):
                if w not in s:
                    lst.append(self._board[h][w])
            new_board[h] = lst

        return Board(np.array(new_board))

    def op_left(self, cut: np.ndarray[int, 2], x: int, y: int) -> Self:
        height = len(cut)
        width = len(cut[0])
        new_board = copy.deepcopy(self._board)
        for h in range(y, y + height):
            if not 0 <= h < self._height:
                continue
            s = set()
            lst = list()
            for w in range(x + width - 1, x - 1, -1):
                if not 0 <= w < self._width:
                    continue
                if cut[h - y][w - x] == 1:
                    lst.append(self._board[h][w])
                    s.add(w)
            for w in range(self._width - 1, -1, -1):
                if w not in s:
                    lst.append(self._board[h][w])
            new_board[h] = list(reversed(lst))

        return Board(np.array(new_board))

    def op_down(self, cut: np.ndarray[int, 2], x: int, y: int) -> Self:
        height = len(cut)
        width = len(cut[0])
        new_board = copy.deepcopy(self._board)

        for w in range(x, x + width):
            if not 0 <= w < self._width:
                continue
            s = set()
            lst = list()
            for h in range(y, y + height):
                if not 0 <= h < self._height:
                    continue
                if cut[h - y][w - x] == 1:
                    lst.append(self._board[h][w])
                    s.add(h)
            for h in range(self._height):
                if h not in s:
                    lst.append(self._board[h][w])

            for h in range(self._height):
                new_board[h][w] = lst[h]

        return Board(np.array(new_board))

    def op_up(self, cut: np.ndarray[int, 2], x: int, y: int) -> Self:
        height = len(cut)
        width = len(cut[0])
        new_board = copy.deepcopy(self._board)

        for w in range(x, x + width):
            if not 0 <= w < self._width:
                continue
            s = set()
            lst = list()
            for h in range(y + height - 1, y - 1, -1):
                if not 0 <= h < self._height:
                    continue
                if cut[h - y][w - x] == 1:
                    lst.append(self._board[h][w])
                    s.add(h)
            for h in range(self._height - 1, -1, -1):
                if h not in s:
                    lst.append(self._board[h][w])

            lst = list(reversed(lst))
            for h in range(self._height):
                new_board[h][w] = lst[h]

        return Board(np.array(new_board))

    def one_up(self, x: int, y: int) -> Self:
        new_board = copy.deepcopy(self._board)
        h, hp = 0, 0
        while h < self._height:
            if h == y:
                h += 1
            new_board[hp][x] = self._board[h][x]
            h += 1
            hp += 1
        new_board[-1][x] = self._board[y][x]

        return Board(np.array(new_board))

    def one_down(self, x: int, y: int) -> Self:
        new_board = copy.deepcopy(self._board)
        h, hp = self._height - 1, self._height - 1
        while h >= 0:
            if h == y:
                h -= 1
            new_board[hp][x] = self._board[h][x]
            h -= 1
            hp -= 1
        new_board[0][x] = self._board[y][x]

        return Board(np.array(new_board))

    def one_left(self, x: int, y: int) -> Self:
        new_board = copy.deepcopy(self._board)
        w, wp = 0, 0
        while w < self._width:
            if w == x:
                w += 1
            new_board[y][wp] = self._board[y][w]
            w += 1
            wp += 1
        new_board[y][-1] = self._board[y][x]

        return Board(np.array(new_board))

    def one_right(self, x: int, y: int) -> Self:
        new_board = copy.deepcopy(self._board)
        w, wp = self._width - 1, self._width - 1
        while w >= 0:
            if w == x:
                w -= 1
            new_board[y][wp] = self._board[y][w]
            w -= 1
            wp -= 1
        new_board[y][0] = self._board[y][x]

        return Board(np.array(new_board))

    def __eq__(self, other: Self) -> bool:
        return np.array_equal(self._board, other.board)

    def clone(self) -> Self:
        new = copy.deepcopy(self._board)
        return Board(new)
