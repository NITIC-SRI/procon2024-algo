# 高専プロコン2024 競技部門

## フォルダ構成

```
.
├── data # 型抜きの情報など，静的ファイルを保存するフォルダ
├── rs # ソースコード
│   ├── board # 盤面の情報を扱う
│   └── utils # その他の便利集
└── strategy # 戦略を記述するフォルダ
```

## ドキュメント

- rust
```bash
cd rs
cargo doc --open
```

### 評価用APIの使い方
以下のコマンドでサーバーが立ち上がる。
```bash
json-server --watch ./data/test.json
```

`http://localhost:3000/{番号}`にアクセスすると本番の仕様と同じ形式でデータが取得できる。

#### json-serverのインストール
```bash
npm install -g json-server
```

