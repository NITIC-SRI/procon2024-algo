import numpy as np
from board import Board

board_grid = np.array(
    [
        [1, 0, 1, 1, 2, 2, 1],
        [2, 3, 1, 1, 0, 0, 2],
        [3, 0, 2, 1, 1, 1, 1],
        [3, 0, 0, 2, 2, 3, 1],
        [2, 2, 3, 2, 0, 2, 2],
        [3, 3, 1, 0, 3, 2, 3],
    ]
)

board = Board(board_grid)

cut_grid = np.array(
    [
        [0, 1, 0],
        [1, 0, 1],
        [1, 1, 0],
    ]
)


def test_op_right():
    new = board.op_right(cut_grid, 1, 2)
    answer = Board(
        [
            [1, 0, 1, 1, 2, 2, 1],
            [2, 3, 1, 1, 0, 0, 2],
            [2, 3, 0, 1, 1, 1, 1],
            [0, 2, 3, 0, 2, 3, 1],
            [2, 3, 2, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
    assert new == answer


def test_op_left():
    new = board.op_left(cut_grid, 1, 2)
    answer = Board(
        [
            [1, 0, 1, 1, 2, 2, 1],
            [2, 3, 1, 1, 0, 0, 2],
            [3, 0, 1, 1, 1, 1, 2],
            [3, 0, 2, 3, 1, 0, 2],
            [2, 2, 0, 2, 2, 2, 3],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
    assert new == answer


def test_op_down():
    new = board.op_down(cut_grid, 1, 2)
    answer = Board(
        [
            [1, 0, 2, 2, 2, 2, 1],
            [2, 2, 3, 1, 0, 0, 2],
            [3, 0, 1, 1, 1, 1, 1],
            [3, 3, 1, 1, 2, 3, 1],
            [2, 0, 0, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
    assert new == answer


def test_op_up():
    new = board.op_up(cut_grid, 1, 2)
    answer = Board(
        [
            [1, 0, 1, 1, 2, 2, 1],
            [2, 3, 1, 1, 0, 0, 2],
            [3, 0, 0, 1, 1, 1, 1],
            [3, 3, 1, 2, 2, 3, 1],
            [2, 0, 2, 0, 0, 2, 2],
            [3, 2, 3, 2, 3, 2, 3],
        ]
    )
    assert new == answer


def test_one_up():
    new = board.one_up(1, 2)
    answer = Board(
        [
            [1, 0, 1, 1, 2, 2, 1],
            [2, 3, 1, 1, 0, 0, 2],
            [3, 0, 2, 1, 1, 1, 1],
            [3, 2, 0, 2, 2, 3, 1],
            [2, 3, 3, 2, 0, 2, 2],
            [3, 0, 1, 0, 3, 2, 3],
        ]
    )
    print(new.board)
    assert new == answer


def test_one_down():
    new = board.one_down(0, 4)
    answer = Board(
        [
            [2, 0, 1, 1, 2, 2, 1],
            [1, 3, 1, 1, 0, 0, 2],
            [2, 0, 2, 1, 1, 1, 1],
            [3, 0, 0, 2, 2, 3, 1],
            [3, 2, 3, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
    assert new == answer


def test_one_left():
    new = board.one_left(1, 2)
    answer = Board(
        [
            [1, 0, 1, 1, 2, 2, 1],
            [2, 3, 1, 1, 0, 0, 2],
            [3, 2, 1, 1, 1, 1, 0],
            [3, 0, 0, 2, 2, 3, 1],
            [2, 2, 3, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
    assert new == answer


def test_one_right():
    new = board.one_right(5, 3)
    answer = Board(
        [
            [1, 0, 1, 1, 2, 2, 1],
            [2, 3, 1, 1, 0, 0, 2],
            [3, 0, 2, 1, 1, 1, 1],
            [3, 3, 0, 0, 2, 2, 1],
            [2, 2, 3, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
    assert new == answer
