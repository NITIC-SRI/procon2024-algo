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


def test_op_up_over1():
    cut = np.array(
        [
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1],
        ]
    )
    new = board.op_up(cut, -1, -2)
    answer = Board(
        [
            [2, 3, 1, 1, 2, 2, 1],
            [3, 0, 1, 1, 0, 0, 2],
            [3, 0, 2, 1, 1, 1, 1],
            [2, 2, 0, 2, 2, 3, 1],
            [3, 3, 3, 2, 0, 2, 2],
            [1, 0, 1, 0, 3, 2, 3],
        ]
    )
    assert new == answer


def test_op_down_over2():
    cut = np.array(
        [
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1],
        ]
    )
    new = board.op_down(cut, 5, 4)
    answer = Board(
        [
            [1, 0, 1, 1, 2, 2, 2],
            [2, 3, 1, 1, 0, 2, 3],
            [3, 0, 2, 1, 1, 2, 1],
            [3, 0, 0, 2, 2, 0, 2],
            [2, 2, 3, 2, 0, 1, 1],
            [3, 3, 1, 0, 3, 3, 1],
        ]
    )
    print(new.board)
    assert new == answer


def test_one_up():
    new = board.get_one_up(1, 2)
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
    new = board.get_one_down(0, 4)
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
    new = board.get_one_left(1, 2)
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
    new = board.get_one_right(5, 3)
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


def test_row_up():
    new = board.clone()
    new._row_up()
    answer = Board(
        [
            [1, 0, 1, 1, 2, 2, 1],
            [2, 3, 1, 1, 0, 0, 2],
            [3, 0, 2, 1, 1, 1, 1],
            [3, 0, 0, 2, 2, 3, 1],
            [2, 2, 3, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
    assert new == answer


def test_fillone():
    end = Board(
        [
            [2, 0, 1, 1, 0, 0, 2],
            [1, 3, 1, 1, 2, 2, 1],
            [3, 0, 2, 1, 1, 1, 1],
            [3, 3, 0, 0, 2, 2, 1],
            [2, 2, 3, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )

    new, action = board.fillone(end)
    print("---------------")
    print(new.board)
    print("---------------")
    print(action, len(action))

    assert new == end
