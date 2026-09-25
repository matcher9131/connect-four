# Connect Four

## 技術スタック

- **Node.js**: v22
- **Next.js**: v15
- **React**: v19
- **Rust**: v1.88
- **wasm-pack**: 最新版（固定）
- **wasm-bindgen**: v0.2.99

## プロジェクト構造

```
connect-four/
├── docker/                     # Docker設定
│   ├── docker-compose.yml      # 開発環境
│   ├── docker-compose.prod.yml # 本番環境
│   ├── Dockerfile.dev          # 開発用イメージ
│   └── Dockerfile              # 本番用イメージ
├── nextjs/                     # Next.jsアプリ
│   ├── src/app/
│   ├── package.json
│   └── next.config.js
└── rust-wasm/                  # Rustプロジェクト
    ├── src/lib.rs
    └── Cargo.toml
```

## セットアップ

### 1. Docker環境の構築

```bash
# 開発用コンテナをビルド・起動
docker-compose -f docker/docker-compose.yml up -d

# コンテナに入る
docker-compose -f docker/docker-compose.yml exec dev bash
```

### 2. 依存関係のインストール（コンテナ内）

```bash
# Next.jsの依存関係をインストール
cd /app/nextjs
npm install

# Rustプロジェクトの初期ビルド（確認用）
cd /app/rust-wasm
cargo build
```

## 開発ワークフロー

### Next.js開発サーバーの起動（コンテナ内）

```bash
cd /app/nextjs
npm run dev
```

ブラウザで http://localhost:3000 にアクセス

### Rust → WASMビルド（コンテナ内）

```bash
# Next.jsディレクトリから
cd /app/nextjs
npm run wasm:build

# リリースビルド（最適化）
npm run wasm:build:release
```

※ Next.js側からの呼び出しの際にファイルコピーが必要な設定になっているため（エラー回避）、rust-wasm側でビルドするのは非推奨

### Rustテストの実行（コンテナ内）

```bash
cd /app/rust-wasm

# 通常のテスト
cargo test

# WASMテスト（ブラウザ環境）
wasm-pack test --headless --firefox
```

## よく使うコマンド

### Docker操作（ホストから）

```bash
# コンテナ起動
docker-compose -f docker/docker-compose.yml up -d

# コンテナに入る
docker-compose -f docker/docker-compose.yml exec dev bash

# コンテナ停止
docker-compose -f docker/docker-compose.yml down

# コンテナ再ビルド
docker-compose -f docker/docker-compose.yml build --no-cache
```

### 開発コマンド（コンテナ内）

```bash
# Next.js開発サーバー
cd /app/nextjs && npm run dev

# WASMビルド
cd /app/nextjs && npm run wasm:build

# Rustテスト
cd /app/rust-wasm && cargo test

# Next.jsビルド
cd /app/nextjs && npm run build

# Next.js本番サーバー
cd /app/nextjs && npm start
```

## 本番環境

### 本番用イメージのビルド

```bash
docker-compose -f docker/docker-compose.prod.yml build
```

### 本番環境の起動

```bash
docker-compose -f docker/docker-compose.prod.yml up -d
```

## トラブルシューティング

### node_modulesが見えない

名前付きボリュームを使用しているため、初回は`npm install`が必要です。

```bash
docker-compose -f docker/docker-compose.yml exec dev bash
cd /app/nextjs
npm install
```

### WASMファイルが読み込めない

1. WASMビルドが完了しているか確認
2. Next.jsを再起動

```bash
cd /app/nextjs
npm run wasm:build
npm run dev
```

### Rustのビルドが遅い

Cargoのキャッシュは名前付きボリュームに保存されています。
初回ビルドは遅いですが、2回目以降は高速化されます。

## VS Code設定

### Remote Containers拡張を使う場合

1. Remote Containers拡張をインストール
2. `docker-compose -f docker/docker-compose.yml up -d`でコンテナを起動
3. VS Codeで「Remote-Containers: Attach to Running Container」を実行
4. `dev`コンテナを選択

### ターミナルから使う場合

```bash
docker-compose -f docker/docker-compose.yml exec dev bash
```

## 次のステップ

1. [rust-wasm/src/lib.rs](rust-wasm/src/lib.rs) にゲームロジックを実装
2. [nextjs/src/app/page.tsx](nextjs/src/app/page.tsx) でWASMを読み込み
3. UIコンポーネントを作成
