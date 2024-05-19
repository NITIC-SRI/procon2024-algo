import torch
import torch.optim as optim
from torch.utils.data import DataLoader
from torch.nn import MSELoss
from torch.utils.data import Dataset
from torchinfo import summary

# 必要なモジュールをインポートします
from model_resnet import FillOneResNN  # モデルのインポート
from dataloader import FillOneDataset  # データセットのインポート

# データセットのインスタンスを作成
dataset = FillOneDataset()
train_size = int(0.8 * len(dataset))
val_size = len(dataset) - train_size
train_dataset, val_dataset = torch.utils.data.random_split(dataset, [train_size, val_size])

train_loader = DataLoader(train_dataset, batch_size=8, shuffle=True, num_workers=4)
val_loader = DataLoader(val_dataset, batch_size=8, shuffle=False, num_workers=4)

# デバイス設定
device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

# モデルのインスタンスを作成し、デバイスに転送
model = FillOneResNN().to(device)

# 損失関数とオプティマイザを定義
criterion = MSELoss()
optimizer = optim.Adam(model.parameters(), lr=0.001)

# モデルのサマリを表示
summary(model, input_size=(32, 9, 256, 256), col_names=["output_size", "num_params"])

# 訓練関数
def train_model(model, train_loader, criterion, optimizer, device):
    model.train()
    running_loss = 0.0
    for i, (inputs, targets) in enumerate(train_loader):
        # データをデバイスに転送
        inputs = torch.stack(inputs, dim=1).to(device).float()
        targets = targets.to(device)

        # 勾配をゼロにする
        optimizer.zero_grad()

        # 順伝播
        outputs = model(inputs)

        # 損失を計算
        loss = criterion(outputs, targets.view(-1, 1))

        # 逆伝播
        loss.backward()

        # パラメータの更新
        optimizer.step()

        # 損失を蓄積
        running_loss += loss.item()

    # 平均損失を計算
    epoch_loss = running_loss / len(train_loader)
    return epoch_loss

# 検証関数
def validate_model(model, val_loader, criterion, device):
    model.eval()
    running_loss = 0.0
    with torch.no_grad():
        for inputs, targets in val_loader:
            # データをデバイスに転送
            inputs = torch.stack(inputs, dim=1).to(device)
            targets = targets.to(device)

            # 順伝播
            outputs = model(inputs)

            # 損失を計算
            loss = criterion(outputs, targets.view(-1, 1))

            # 損失を蓄積
            running_loss += loss.item()

    # 平均損失を計算
    epoch_loss = running_loss / len(val_loader)
    return epoch_loss

# 学習ループ
num_epochs = 1000
best_val_loss = float('inf')

for epoch in range(num_epochs):
    train_loss = train_model(model, train_loader, criterion, optimizer, device)
    val_loss = validate_model(model, val_loader, criterion, device)

    print(f'Epoch {epoch+1}/{num_epochs}, Train Loss: {train_loss:.4f}, Val Loss: {val_loss:.4f}')

    # モデルの保存
    if val_loss < best_val_loss:
        best_val_loss = val_loss
        torch.save(model.state_dict(), 'best_model.pth')
        print(f'Saved Best Model with Val Loss: {best_val_loss:.4f}')
