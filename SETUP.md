# セットアップ手順

このガイドでは、初めてプロジェクトをセットアップする手順を説明します。

## 前提条件

- Docker Desktop for Windowsがインストールされていること
- VS Codeがインストールされていること（推奨）

## ステップ1: Docker環境の構築

```bash
# プロジェクトディレクトリに移動
cd f:\Documents\Workspace\Nodejs\connect-four

# 開発用コンテナをビルド・起動
docker-compose -f docker/docker-compose.yml up -d
```

初回ビルドは10-15分程度かかります（Rust、Node.js、wasm-packのインストール）。

## ステップ2: 依存関係のインストール

### Next.jsの依存関係

```bash
# コンテナに入る
docker-compose -f docker/docker-compose.yml exec dev bash

# Next.jsディレクトリへ移動
cd /app/nextjs

# 依存関係をインストール
npm install
```

### Rustの確認

```bash
# Rustディレクトリへ移動
cd /app/rust-wasm

# ビルドテスト
cargo build

# テスト実行
cargo test
```

すべて成功すれば環境構築完了です。

## ステップ3: 開発サーバーの起動

### ターミナル1: Next.js開発サーバー

```bash
# コンテナ内で
cd /app/nextjs
npm run dev
```

ブラウザで http://localhost:3000 にアクセスして確認。

### ターミナル2: WASMビルド（必要に応じて）

```bash
# コンテナ内で
cd /app/nextjs
npm run wasm:build
```

## VS Codeでの開発

### 方法1: Remote Containers拡張（推奨）

1. **Remote - Containers** 拡張をインストール
2. コンテナが起動している状態で、VS Codeのコマンドパレット（Ctrl+Shift+P）を開く
3. 「Remote-Containers: Attach to Running Container」を実行
4. `connect-four-dev-1`（または類似の名前）を選択
5. `/app`フォルダを開く

### 方法2: ローカルでファイル編集

ホストのVS Codeで`f:\Documents\Workspace\Nodejs\connect-four`を開き、
ターミナルでDockerコンテナに接続：

```bash
docker-compose -f docker/docker-compose.yml exec dev bash
```

ファイル変更は自動的にコンテナ内に反映されます。

## 動作確認

### 1. Next.jsの確認

http://localhost:3000 にアクセスして「Connect Four」ページが表示されることを確認。

### 2. WASMの確認

```bash
# コンテナ内で
cd /app/rust-wasm
wasm-pack build --target web --out-dir pkg

# ビルド成功を確認
ls -la pkg/
```

`connect_four_wasm.js`、`connect_four_wasm_bg.wasm`などが生成されていればOK。

### 3. Rustテストの確認

```bash
cd /app/rust-wasm
cargo test
```

すべてのテストがパスすればOK。

## トラブルシューティング

### Dockerコンテナが起動しない

```bash
# ログを確認
docker-compose -f docker/docker-compose.yml logs

# コンテナを再ビルド
docker-compose -f docker/docker-compose.yml down
docker-compose -f docker/docker-compose.yml build --no-cache
docker-compose -f docker/docker-compose.yml up -d
```

### npm installが失敗する

```bash
# コンテナを再起動
docker-compose -f docker/docker-compose.yml restart

# node_modulesボリュームを削除して再作成
docker-compose -f docker/docker-compose.yml down -v
docker-compose -f docker/docker-compose.yml up -d
docker-compose -f docker/docker-compose.yml exec dev bash
cd /app/nextjs && npm install
```

### Rustのビルドが失敗する

```bash
# Cargoキャッシュをクリア
docker-compose -f docker/docker-compose.yml exec dev bash
cd /app/rust-wasm
cargo clean
cargo build
```

## 次のステップ

セットアップが完了したら、[README.md](README.md)を参照して開発を進めてください。
