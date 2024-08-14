from dataclasses import dataclass


@dataclass
class RuleConfig:
    board_size: int = 8
    mask_count: int = 1 + 2 * 3  # 1x1 + (2x2 + 4x4) * 3

    @property
    def action_count(self) -> int:
        return self.mask_count * 4 * self.board_size**2

    @property
    def direction_count(self) -> int:
        return 4


@dataclass
class MCTConfig:
    simulation: int = 100
    noise_alpha: float = 0.9
    noise_frac: float = 0.25
    cpuct: float = 1.1


rule_config = RuleConfig()
mct_config = MCTConfig()
