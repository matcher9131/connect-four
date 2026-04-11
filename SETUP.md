# セットアップ手順

このガイドでは、初めてプロジェクトをセットアップする手順を説明します。

## 前提条件

- Docker Desktop for Windowsがインストールされていること
- WSL2 (Ubuntu推奨) がインストールされていること
- VS Codeがインストールされていること（推奨）
  - VS Code拡張: **WSL** (推奨)

## 重要: WSLを使用する理由

**WindowsドライブでDockerを使うとI/O性能が大幅に低下します。**
Cargo/Rustビルドやnpm installなどのI/O集中操作を高速化するため、**WSL2のLinuxファイルシステム上でプロジェクトを配置することを強く推奨します。**

- Windowsドライブ (`F:\`, `C:\` など): 遅い
- WSLファイルシステム (`~/workspace/`): 10倍以上高速

## プロジェクトの配置

### Windows側にある場合（移行が必要）

```bash
# WSLを起動
wsl -d Ubuntu

# プロジェクトをWSLにコピー
mkdir -p ~/workspace
cp -r /mnt/f/Documents/Workspace/Nodejs/connect-four ~/workspace/
cd ~/workspace/connect-four
```

### 既にWSL側にある場合

```bash
# WSLを起動してプロジェクトディレクトリに移動
wsl -d Ubuntu
cd ~/workspace/connect-four
```

## ステップ1: Docker環境の構築

**重要: 以下のコマンドはWSL内で実行してください。**

```bash
# WSL内でプロジェクトディレクトリに移動
cd ~/workspace/connect-four

# 開発用コンテナをビルド・起動
docker-compose -f docker/docker-compose.yml up -d
```

初回ビルドは10-15分程度かかります（Rust、Node.js、wasm-packのインストール）。

## ステップ2: 依存関係のインストール

### Next.jsの依存関係

**重要: 以下のコマンドはWSL内で実行してください。**

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

## VS Codeでの開発（WSL推奨）

### 方法1: WSL拡張を使う（最推奨）

1. VS Codeに **WSL** 拡張をインストール
2. VS Codeのコマンドパレット（Ctrl+Shift+P）を開く
3. 「WSL: Connect to WSL」を実行
4. WSL内のVS Codeで `~/workspace/connect-four` を開く
5. WSL内のターミナルでDockerコマンドを実行

**メリット**: 最高のパフォーマンス、Linuxネイティブな開発体験

### 方法2: Remote Containers拡張

1. **Remote - Containers** 拡張をインストール
2. コンテナが起動している状態で、VS Codeのコマンドパレット（Ctrl+Shift+P）を開く
3. 「Remote-Containers: Attach to Running Container」を実行
4. `connect-four-dev-1`（または類似の名前）を選択
5. `/app`フォルダを開く

### 方法3: WSLターミナル経由

WSL内でVS Codeを起動：

```bash
# WSL内で
cd ~/workspace/connect-four
code .
```

### Windowsエクスプローラーからアクセス

WSL内のファイルはWindowsエクスプローラーから以下のパスでアクセス可能：

```
\\wsl$\Ubuntu\home\<username>\workspace\connect-four
```

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

### I/O性能が遅い（Windows警告が出る）

プロジェクトがWindowsドライブ (`F:\`, `C:\`) にある場合、WSLに移行してください：

```bash
# WSL起動
wsl -d Ubuntu

# プロジェクトをコピー
mkdir -p ~/workspace
cp -r /mnt/f/Documents/Workspace/Nodejs/connect-four ~/workspace/
cd ~/workspace/connect-four

# Dockerコンテナを起動
docker-compose -f docker/docker-compose.yml up -d
```

### Dockerコンテナが起動しない

**WSL内で実行してください：**

```bash
# ログを確認
docker-compose -f docker/docker-compose.yml logs

# コンテナを再ビルド
docker-compose -f docker/docker-compose.yml down
docker-compose -f docker/docker-compose.yml build --no-cache
docker-compose -f docker/docker-compose.yml up -d
```

### npm installが失敗する

**WSL内で実行してください：**

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

**WSL内で実行してください：**

```bash
# Cargoキャッシュをクリア
docker-compose -f docker/docker-compose.yml exec dev bash
cd /app/rust-wasm
cargo clean
cargo build
```

## 次のステップ

セットアップが完了したら、[README.md](README.md)を参照して開発を進めてください。
