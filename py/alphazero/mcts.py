from py.alphazero.board import Board
from py.alphazero.model import PVNet
from py.alphazero.config import MCTConfig, RuleConfig


class MCT:
    def __init__(
        self, model: PVNet, rule_config: RuleConfig, mcts_config: MCTConfig
    ) -> None:
        self.model = model
        self.device = next(model.parameters()).device

        self.rule = rule_config
        self.config = mcts_config

        self.P = {}
        self.N = {}
        self.W = {}
        self.transition: dict[Board, list[Board]] = {}

    def search(self, board: Board) -> list[float]:
        pass

    def expand(self, board: Board) -> float:
        pass

    def evaluate(self, board: Board) -> float:
        pass
