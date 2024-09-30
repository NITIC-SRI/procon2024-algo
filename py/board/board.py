import copy
from typing import List, Self, Tuple

import numpy as np

Direction = {"up": 0, "right": 1, "down": 2, "left": 3}


class Board:
    def __init__(self, board: np.ndarray[int, 2]) -> None:
        self._board = board
        self._height = len(board)
        self._width = len(board[0])

    @classmethod
    def random_board(cls, height: int, width: int) -> Self:
        board = np.random.randint(4, size=(height, width))
        return cls(board)

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
            # 　範囲外の場合はスキップ
            if not 0 <= h < self._height:
                continue

            s = set()  # 型抜きした場所を記録
            lst = list()  # 新しい行
            for w in range(x, x + width):
                if not 0 <= w < self._width:
                    continue

                # 型抜きしたものを先頭に追加していく
                if cut[h - y][w - x] == 1:
                    lst.append(self._board[h][w])
                    s.add(w)

            # 型抜きしていないものを後ろに追加
            for w in range(self._width):
                if w not in s:
                    lst.append(self._board[h][w])

            # 新しい行を代入
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

    # 0番の型で操作するメソッド
    def get_one_up(self, x: int, y: int) -> Self:
        new_board = copy.deepcopy(self._board)
        if y == self._height - 1:
            return Board(np.array(new_board))
        h, hp = 0, 0
        while h < self._height:
            if h == y:
                h += 1
            new_board[hp][x] = self._board[h][x]
            h += 1
            hp += 1
        new_board[-1][x] = self._board[y][x]

        return Board(np.array(new_board))

    def get_one_down(self, x: int, y: int) -> Self:
        new_board = copy.deepcopy(self._board)
        if y == 0:
            return Board(np.array(new_board))
        h, hp = self._height - 1, self._height - 1
        while h >= 0:
            if h == y:
                h -= 1
            new_board[hp][x] = self._board[h][x]
            h -= 1
            hp -= 1
        new_board[0][x] = self._board[y][x]

        return Board(np.array(new_board))

    def get_one_left(self, x: int, y: int) -> Self:
        new_board = copy.deepcopy(self._board)
        if x == self._width - 1:
            return Board(np.array(new_board))
        w, wp = 0, 0
        while w < self._width:
            if w == x:
                w += 1
            new_board[y][wp] = self._board[y][w]
            w += 1
            wp += 1
        new_board[y][-1] = self._board[y][x]

        return Board(np.array(new_board))

    def get_one_right(self, x: int, y: int) -> Self:
        new_board = copy.deepcopy(self._board)
        if x == 0:
            return Board(np.array(new_board))
        w, wp = self._width - 1, self._width - 1
        while w >= 0:
            if w == x:
                w -= 1
            new_board[y][wp] = self._board[y][w]
            w -= 1
            wp -= 1
        new_board[y][0] = self._board[y][x]

        return Board(np.array(new_board))

    def _one_up(self, x: int, y: int) -> None:
        if y == self._height - 1:
            return
        e = self.board[y][x]
        h, hp = 0, 0
        while h < self._height:
            if h == y:
                h += 1
            self._board[hp][x] = self._board[h][x]
            h += 1
            hp += 1
        self._board[-1][x] = e

    def _one_down(self, x: int, y: int) -> None:
        if y == 0:
            return
        e = self.board[y][x]
        h, hp = self._height - 1, self._height - 1
        while h >= 0:
            if h == y:
                h -= 1
            self._board[hp][x] = self._board[h][x]
            h -= 1
            hp -= 1
        self._board[0][x] = e

    def _one_left(self, x: int, y: int) -> None:
        if x == self._width - 1:
            return
        e = self.board[y][x]
        w, wp = 0, 0
        while w < self._width:
            if w == x:
                w += 1
            self._board[y][wp] = self._board[y][w]
            w += 1
            wp += 1
        self._board[y][-1] = e

    def _one_right(self, x: int, y: int) -> None:
        if x == 0:
            return
        e = self.board[y][x]
        w, wp = self._width - 1, self._width - 1
        while w >= 0:
            if w == x:
                w -= 1
            self._board[y][wp] = self._board[y][w]
            w -= 1
            wp -= 1
        self._board[y][0] = e

    def _row_up(self, row_num: int) -> None:
        head = copy.deepcopy(self._board[0:row_num])

        for h in range(self._height - row_num):
            self._board[h] = self._board[h + row_num]
        self._board[-row_num:] = head

    def fillone(self, end: Self) -> Tuple[Self, List]:
        new = self.clone()
        actions = []
        for y in range(self._height):
            for x in range(self._width):

                is_break = False
                for w in range(self._width - x):
                    if end.board[y][x] == new.board[0][w]:
                        new._one_left(w, 0)
                        actions.append((w, 0, 0, "left"))
                        is_break = True
                        break

                if is_break:
                    continue

                for h in range(1, self._height - y):
                    for w in range(self._width - x):
                        if end.board[y][x] == new.board[h][w]:
                            new._one_down(w, h)
                            actions.append((w, h, 0, "down"))
                            new._one_left(w, 0)
                            actions.append((w, 0, 0, "left"))
                            is_break = True
                            break
                    if is_break:
                        break

                if is_break:
                    continue

                for h in range(1, self._height - y):
                    for w in range(self.width - x, self._width):
                        if end.board[y][x] == new.board[h][w]:
                            new._one_right(w, h)
                            actions.append((w, h, 0, "right"))
                            new._one_down(0, h)
                            actions.append((0, h, 0, "down"))
                            new._one_left(0, 0)
                            actions.append((0, 0, 0, "left"))
                            is_break = True
                            break

                    if is_break:
                        break

            new._row_up()
            actions.append((0, -255, 22, 'rowup'))

        return new, actions

    def __eq__(self, other: Self) -> bool:
        return np.array_equal(self._board, other.board)

    def clone(self) -> Self:
        new = copy.deepcopy(self._board)
        return Board(new)
