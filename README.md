# Image Compression Benchmark

画像フォーマット（JPG/PNG/WEBP）および圧縮フォーマット（ZIP/TAR.GZ/ZSTD/XZ/7Z）のベンチマーク結果を可視化するプロジェクトです。

## 可視化結果

**GitHub Pages**: [https://YOUR_USERNAME.github.io/datamanagement-image-compression-bench/](https://YOUR_USERNAME.github.io/datamanagement-image-compression-bench/)

※ リポジトリの Settings → Pages で GitHub Actions からのデプロイを有効にしてください。

## ローカルでのビルド

### 方法1: コンテナ使用（推奨）

Podman または Docker を使用してビルドできます：

```bash
# コンテナイメージをビルドしてレンダリング
make render

# Docker を使用する場合
CONTAINER_ENGINE=docker make render
```

### 方法2: ローカル環境

```bash
# 依存をインストール
uv sync

# Quarto をインストール（未インストールの場合）
curl -LO https://github.com/quarto-dev/quarto-cli/releases/download/v1.7.31/quarto-1.7.31-linux-amd64.tar.gz
mkdir -p ~/.local/bin && tar -xzf quarto-1.7.31-linux-amd64.tar.gz
mv quarto-1.7.31 ~/.local/quarto && ln -sf ~/.local/quarto/bin/quarto ~/.local/bin/quarto
export PATH="$HOME/.local/bin:$PATH"

# レンダリング
make render-local

# プレビュー（ブラウザで確認）
make preview
```

## プロジェクト構成

```
.
├── docs/                    # Quarto ドキュメント
│   ├── _quarto.yml          # Quarto 設定
│   ├── index.qmd            # トップページ
│   └── visualization.qmd    # 可視化ページ
├── result_csv/              # ベンチマーク結果 CSV
├── .github/workflows/       # GitHub Actions
│   └── publish.yml          # GitHub Pages 自動デプロイ
├── Containerfile            # Podman/Docker 用
├── Makefile                 # ビルドコマンド
└── pyproject.toml           # Python 依存定義
```

## GitHub Pages の設定

1. リポジトリの Settings → Pages を開く
2. Source で "GitHub Actions" を選択
3. main ブランチに push すると自動でデプロイされる

## 依存管理

Python の依存は [uv](https://docs.astral.sh/uv/) で管理しています：

```bash
# 依存の同期
uv sync

# 依存の追加
uv add <package>

# 依存の更新
uv lock --upgrade && uv sync
```

## 開発ツール

```bash
# Lint
uv run ruff check --fix .

# フォーマット
uv run ruff format .

# 型チェック
uv run mypy .
```
