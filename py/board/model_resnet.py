import torch
from torch import nn
import torch.nn.functional as F

I_CH = 9
L_CH = 128
O_CH = 100
LN_CH = 1

class FillOneResNN(nn.Module):
    def __init__(self) -> None:
        super().__init__()

        self.layer_input = nn.Sequential(
            nn.Conv2d(I_CH, L_CH, kernel_size=3, padding=1),
            nn.BatchNorm2d(L_CH),
            nn.ReLU(),
        )
        
        self.layer_1 = nn.Sequential(
            nn.Conv2d(L_CH, L_CH, kernel_size=3, padding=1),
            nn.BatchNorm2d(L_CH),
            nn.ReLU()
        )

        self.layer_2 = nn.Sequential(
            nn.Conv2d(L_CH, L_CH, kernel_size=3, padding=1),
            nn.BatchNorm2d(L_CH),
            nn.ReLU()
        )

        self.layer_3 = nn.Sequential(
            nn.Conv2d(L_CH, L_CH, kernel_size=3, padding=1),
            nn.BatchNorm2d(L_CH),
            nn.ReLU()
        )

        self.layer_4 = nn.Sequential(
            nn.Conv2d(L_CH, L_CH, kernel_size=3, padding=1),
            nn.BatchNorm2d(L_CH),
            nn.ReLU()
        )

        self.layer_5 = nn.Sequential(
            nn.Conv2d(L_CH, L_CH, kernel_size=3, padding=1),
            nn.BatchNorm2d(L_CH),
            nn.ReLU()
        )

        self.layer_output = nn.Sequential(
            nn.Conv2d(L_CH, O_CH, kernel_size=3, padding=1),
            nn.BatchNorm2d(O_CH),
            nn.AdaptiveAvgPool2d((1, 1)),
            nn.Flatten(),
            nn.Linear(O_CH, LN_CH)
        )

    def forward(self, x):
        x = self.layer_input(x)
        x_skip = x.clone()

        x = self.layer_1(x)
        x = self.layer_2(x)
        x = self.layer_3(x)
        x = self.layer_4(x)
        x = self.layer_5(x)

        x = self.layer_output(x + x_skip)
        return x

if __name__=="__main__":
    from torchinfo import summary

    device = torch.device("cuda")
    model = FillOneResNN().to(device)

    summary(model, input_size=(32, 9, 256, 256), col_names=["output_size", "num_params"])

