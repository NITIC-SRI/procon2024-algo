import torch
from torch import nn
from config import rule_config

BOARD_SIZE = rule_config.board_size
ACTION_COUNT = rule_config.action_count


class PVNet(nn.Module):
    def __init__(self) -> None:
        super().__init__()

        self.layers = nn.Sequential(
            ResBlock(5 + ACTION_COUNT, 128, 128),
            ResBlock(128, 128, 256),
            ResBlock(256, 256, 512),
        )

        self.policy_head = nn.Sequential(
            nn.Conv2d(512, 128, kernel_size=3, padding=1),
            nn.BatchNorm2d(128),
            nn.ReLU(),
            nn.Flatten(),
            nn.Linear(128 * BOARD_SIZE**2, ACTION_COUNT),
        )

        self.value_head = nn.Sequential(
            nn.Conv2d(512, 128, kernel_size=3, padding=1),
            nn.BatchNorm2d(128),
            nn.ReLU(),
            nn.Flatten(),
            nn.Linear(128 * BOARD_SIZE**2, 32 * BOARD_SIZE**2),
            nn.ReLU(),
            nn.Linear(32 * BOARD_SIZE**2, 1),
            nn.Tanh(),
        )

    def forward(self, x: torch.Tensor) -> tuple[torch.Tensor, torch.Tensor]:
        x = self.layers(x)
        p = self.policy_head(x)
        v = self.value_head(x)
        return p, v


class ResBlock(nn.Module):
    def __init__(self, in_channels: int, mid_channels: int, out_channels: int) -> None:
        super().__init__()

        self.layers = nn.Sequential(
            nn.Conv2d(in_channels, mid_channels, kernel_size=3, padding=1),
            nn.BatchNorm2d(mid_channels),
            nn.ReLU(),
            nn.Conv2d(mid_channels, out_channels, kernel_size=3, padding=1),
            nn.BatchNorm2d(out_channels),
        )

        self.ds = nn.Sequential(
            nn.Conv2d(in_channels, out_channels, kernel_size=1),
            nn.BatchNorm2d(out_channels),
        )

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        skip_x = self.ds(x.clone())
        x = self.layers(x)
        return x + skip_x


if __name__ == "__main__":
    import torch
    from torchinfo import summary

    arr = torch.randn((1, 5 + ACTION_COUNT, BOARD_SIZE, BOARD_SIZE))
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    model = PVNet().to(device)
    arr = arr.to(device)
    p, v = model(arr)

    summary(model, input_size=(1, 5 + ACTION_COUNT, BOARD_SIZE, BOARD_SIZE))
