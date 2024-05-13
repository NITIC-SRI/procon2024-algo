import tkinter as tk

import numpy as np
from board.board import Board

root = tk.Tk()
root.title("迷路")
canvas = tk.Canvas(width=800, height=560, bg="white")
canvas.pack()

move = False

start = Board(
    np.array(
        [
            [1, 0, 1, 1, 2, 2, 1],
            [2, 3, 1, 1, 0, 0, 2],
            [3, 0, 2, 1, 1, 1, 1],
            [3, 0, 0, 2, 2, 3, 1],
            [2, 2, 3, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
)

end = Board(
    np.array(
        [
            [2, 0, 1, 1, 0, 0, 2],
            [1, 3, 1, 1, 2, 2, 1],
            [3, 0, 2, 1, 1, 1, 1],
            [3, 3, 0, 0, 2, 2, 1],
            [2, 2, 3, 2, 0, 2, 2],
            [3, 3, 1, 0, 3, 2, 3],
        ]
    )
)

_, actions = start.fillone(end)


def switch():
    global move
    move = not move


now = 0
fillx, filly = 0, 0


def render():
    global now
    global actions
    if move:
        for y in range(start.height):
            for x in range(start.width):
                if y == actions[now][1] and x == actions[now][0]:
                    color = "red"
                else:
                    color = "white"
                canvas.create_rectangle(
                    x * 80, y * 80, x * 80 + 80, y * 80 + 80, fill=color
                )

                canvas.create_text(
                    x * 80 + 40,
                    y * 80 + 40,
                    text=start.board[y][x],
                    font=("", 40),
                )

        if now < len(actions):
            action = actions[now]
            if action[2] == "down":
                start._one_down(action[0], action[1])
            elif action[2] == "left":
                start._one_left(action[0], action[1])
            elif action[2] == "right":
                start._one_right(action[0], action[1])
            elif action[2] == "rowup":
                start._row_up()
            now += 1

    root.after(2000, render)


button = tk.Button(root, textvariable="button", command=switch)
button.pack()
render()
root.mainloop()
