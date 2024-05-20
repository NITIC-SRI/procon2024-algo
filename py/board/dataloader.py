from torch.utils.data import Dataset, DataLoader, random_split
from fillone_minimal import compress_actions

import numpy as np

from board import Board
import random

# ratio generator
def ratio(i):
    if i == 0:
        return 10
    else:
        return - i + 200

RATIO = np.array([ratio(x) for x in range(0, 101)])
RATE = RATIO / RATIO.sum()
ELEMS = [i for i in range(0, 101)]

# 0 padding
def zero_board_padding(board: Board) -> list:
    b = np.zeros((256, 256), dtype=np.float32)
    for y in range(board.height):
        for x in range(board.width):
            b[y][x] = board.board[y][x]
    return b

# mask board
def mask_board(h, w):
    b = np.zeros((256, 256), dtype=np.float32)
    for y in range(h):
        for x in range(w):
            b[y][x] = 1.0
    return b

def split_layers(board: Board):
    mask = mask_board(board.height, board.width)

    b = zero_board_padding(board)
    b0 = (b == 0).astype(np.float32) * mask
    b1 = (b == 1).astype(np.float32)
    b2 = (b == 2).astype(np.float32)
    b3 = (b == 3).astype(np.float32)

    return mask, b0, b1, b2, b3

class FillOneDataset(Dataset):
    def __getitem__(self, index):
        # generate baord data
        W = random.randint(1, 256)
        H = random.randint(1, 256)
        start = Board.random_board(W, H)
        end = start.clone()

        count = np.random.choice(ELEMS, 1, p=RATE)[0]

        for _ in range(count):
            cut = np.random.randint(
                0, 2, (random.randint(1, 256), random.randint(1, 256))
            )
            to = random.randint(0, 4)
            x = np.random.randint(-len(cut[0]) + 1, W)
            y = np.random.randint(-len(cut) + 1, H)

            if to == 0:
                end = end.op_up(cut, x, y)
            elif to == 1:
                end = end.op_down(cut, x, y)
            elif to == 2:
                end = end.op_left(cut, x, y)
            elif to == 3:
                end = end.op_right(cut, x, y)
        
        _, actions = start.fillone(end)
        actions = compress_actions(H, W, actions)

        mask_s, b0s, b1s, b2s, b3s = split_layers(start)
        _, b0e, b1e, b2e, b3e = split_layers(end)

        score = len(actions) / (W * H)

        return [mask_s, b0s, b1s, b2s, b3s, b0e, b1e, b2e, b3e], float(score)
    
    def __len__(self):
        return 1000
    

if __name__=="__main__":
    dataset = FillOneDataset()
    dataloader = DataLoader(dataset, batch_size=32, num_workers=24)

    for (x, y) in dataloader:
        print(x, y)