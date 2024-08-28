import math
import numpy as np
import torch

from board import Board
from model import PVNet
from masks import gen_const_mask
from config import MCTConfig, RuleConfig, rule_config
from dataloader import split_layers

DIRECTIONS = [
    0,  # up
    1,  # right
    2,  # down
    3,  # left
]

FILTERS = [
    [[1]],
]

for i in [2, 4]:
    FILTERS.append(gen_const_mask(2**i, 0))
    FILTERS.append(gen_const_mask(2**i, 1))
    FILTERS.append(gen_const_mask(2**i, 2))

_a = []

for f in FILTERS:
    for d in DIRECTIONS:
        for y in range(rule_config.board_size):
            for x in range(rule_config.board_size):
                _a.append((f, d, y, x))


ACTIONS = {i: a for i, a in enumerate(_a)}


class MCT:
    def __init__(
        self, model: PVNet, rule_config: RuleConfig, mcts_config: MCTConfig
    ) -> None:
        self.model = model
        self.device = next(model.parameters()).device

        self.rule = rule_config
        self.config = mcts_config

        self.P: dict[Board, list[Board]] = {}
        self.N: dict[Board, list[int]] = {}
        self.W: dict[Board, list[float]] = {}
        self.transition: dict[Board, list[Board]] = {}

    def C_puct(self, state: Board) -> float:
        return self.config.cpuct

    def N_s(self, state: Board) -> int | None:
        if state not in self.N:
            return None
        return sum(self.N[state])

    def N_sa(self, state: Board, action: int) -> int | None:
        if state not in self.N:
            return None
        return self.N[state][action]

    def search(self, state: Board, goal: Board) -> list[float]:
        if state == goal:
            return [0.0] * len(ACTIONS)

        if self.N_s(state) is None:
            self.expand(state)

        dnoise = np.random.dirichlet(alpha=[self.config.noise_alpha] * len(ACTIONS))
        for a, noise in zip(ACTIONS.keys(), dnoise):
            self.P[state][a] = (1 - self.config.noise_frac) * self.P[state][
                a
            ] + self.config.noise_frac * noise

        for _ in range(self.config.simulation):
            U = [
                self.C_puct(state)
                * self.P[state][a]
                * math.sqrt(self.N_s(state))
                / (1 + self.N_sa(state, a))
                for a in ACTIONS.keys()
            ]

            Q = [q / n if n != 0 else q for q, n in zip(self.W[state], self.N[state])]

            scores = [q + u for q, u in zip(Q, U)]
            action = np.argmin(scores)

            next_state = self.transition[state][action]
            value = self.evaluate(next_state)

            self.N[state][action] += 1
            self.W[state][action] += value

        return [n / self.N_s(state) for n in self.N[state]]

    def expand(self, board: Board) -> float:
        self.N[board] = [0] * len(ACTIONS)
        self.W[board] = [0] * len(ACTIONS)
        self.transition[board] = [None] * len(ACTIONS)

        next_states = []

        for a, (f, d, y, x) in ACTIONS.items():
            next_board: Board = board.clone()
            if d == 0:
                next_board.op_up(f, x, y)
            elif d == 1:
                next_board.op_right(f, x, y)
            elif d == 2:
                next_board.op_down(f, x, y)
            elif d == 3:
                next_board.op_left(f, x, y)

            self.transition[board][a] = next_board
            next_states.append(next_board)

        with torch.no_grad():
            mask, b0, b1, b2, b3 = split_layers(next_board, 256)
            input_tensor = torch.stack([mask, b0, b1, b2, b3]).to(self.device)

            p, v = self.model(input_tensor)
            policy = p.cpu().flatten().tolist()
            value = v.cpu().item()

        self.P[board] = policy

        return value

    def evaluate(self, state: Board, goal: Board) -> float:
        if state == goal:
            return 0.0
        elif self.N_s(state) is None:
            return self.expand(state)
        else:
            U = [
                self.C_puct(state)
                * self.P[state][a]
                * math.sqrt(self.N_s(state))
                / (1 + self.N_sa(state, a))
                for a in ACTIONS.keys()
            ]

            Q = [q / n if n != 0 else q for q, n in zip(self.W[state], self.N[state])]

            scores = [q + u for q, u in zip(Q, U)]
            action = np.argmin(scores)

            next_state = self.transition[state][action]
            value = self.evaluate(next_state)

            self.N[state][action] += 1
            self.W[state][action] += value

            return value
