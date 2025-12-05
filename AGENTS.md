# AGENTS: Python 可視化パートの作業指針

このファイルは、本リポジトリにおける CSV 可視化部分に関わるエージェント向けの指針です。範囲はリポジトリ直下をルートとする全体です。

## 目的 / ゴール
- CSV（`result_csv/` 配下）を読み込み、JPG/PNG/WEBP などフォーマット別にメトリクスを可視化する。
- Long 形式（`format` 列あり）と Wide 形式（列名に `jpg|jpeg|png|webp` を含む複数列）の両方に対応し、自動検出を試みる。
- **GitHub Pages で公開**（Quarto → HTML 出力）を提供する。
- インタラクティブな分析が必要な場合は Quarto HTML のコード表示機能を利用。

## 可視化の方法

### Quarto ドキュメント（GitHub Pages 公開）
- `docs/visualization.qmd`: ソースファイル（Python コード + Markdown）
- `docs/_site/`: 生成された HTML（GitHub Pages で公開）
- `docs/_quarto.yml`: Quarto 設定（website 形式）

### ビルド方法

**コンテナ使用（推奨）:**
```bash
make render
# Docker の場合
CONTAINER_ENGINE=docker make render
```

**ローカル環境:**
```bash
export PATH="$HOME/.local/bin:$PATH"
export QUARTO_PYTHON="$(pwd)/.venv/bin/python"
uv sync
cd docs && quarto render
```

## 主要ファイル
- `docs/visualization.qmd`: Quarto ソース（メイン可視化）
- `docs/index.qmd`: トップページ
- `docs/_quarto.yml`: Quarto 設定（website + HTML 出力）
- `pyproject.toml`: uv が参照する依存定義と開発用グループ（dev）
- `uv.lock`: ロックファイル（コミット対象）
- `Containerfile`: Podman/Docker 用ビルドイメージ
- `Makefile`: ビルドコマンド
- `.github/workflows/publish.yml`: GitHub Pages 自動デプロイ

## 実行方法（ローカル / CI）
- 依存同期: `uv sync`

### コンテナでビルド（推奨）
```bash
make render
```

### ローカルでビルド
```bash
make render-local
```

### ローカルプレビュー
```bash
make preview
```

## 対応するデータ形式
- Long: `format` 列があり、値に JPG/JPEG/PNG/WEBP を含む。
- Wide: 列名に `jpg|jpeg|png|webp` を含む複数の数値列を自動で縦持ち化（melt）。

## 必要な開発ツール（uv 管理 / 2025 年ベストプラクティス）
- 本番依存:
  - `pandas`, `pyarrow`（データ処理）
  - `matplotlib`（グラフ描画）
  - `jupyter`, `nbformat`, `nbclient`（Quarto 実行用）
  - `tabulate`（テーブル出力）
- 開発依存（dev グループ）:
  - `ruff`（lint, format）
  - `mypy`（型チェック）
  - `pytest`（テスト）
- システムツール:
  - `quarto`（ローカルインストール、または Containerfile 経由）
  - `podman` または `docker`（コンテナビルド用）
- 代表コマンド:
  - Lint+自動修正: `uv run ruff check --fix .`
  - フォーマット: `uv run ruff format .`
  - 型チェック: `uv run mypy .`
  - テスト: `uv run pytest -q`
  - Quarto レンダリング: `make render`

## 依存の追加/更新
- 本番依存: `uv add <pkg>`
- 開発依存: `uv add --group dev <pkg>`
- アップグレード: `uv lock --upgrade` → `uv sync`

## コーディング/変更方針
- 配布用のビルド対応は不要（ドキュメント生成用途）。
- 既存のスタイル/構成を尊重し、必要最小限の変更に留める。
- 追加 CSV に備え、列名のばらつきやフォーマット名の同義（JPG/JPEG）を正規化する。
- 大きな変更前に README の手順が崩れないかを確認する。

## トラブルシューティング

### コンテナ関連
- Podman がない場合: `CONTAINER_ENGINE=docker make render`
- SELinux 環境: `:Z` オプションが Makefile に含まれています

### Quarto 関連
- `quarto` コマンドが見つからない: `~/.local/bin` を PATH に追加
- Python 環境が見つからない: `QUARTO_PYTHON` 環境変数で `.venv/bin/python` を指定
- モジュールが見つからない: `uv sync` で依存を同期

### GitHub Pages 関連
- デプロイされない: Settings → Pages で "GitHub Actions" を選択
- ビルド失敗: Actions タブでログを確認
