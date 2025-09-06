# AGENTS: Python 可視化パートの作業指針

このファイルは、本リポジトリにおける「Python による CSV 可視化（Streamlit）」部分に関わるエージェント向けの指針です。範囲はリポジトリ直下をルートとする全体です。

## 目的 / ゴール
- CSV（例: `image_comparison_format_results.csv`）を読み込み、JPG/PNG/WEBP などフォーマット別にメトリクスを可視化する。
- Long 形式（`format` 列あり）と Wide 形式（列名に `jpg|jpeg|png|webp` を含む複数列）の両方に対応し、自動検出を試みる。
- 将来的に CSV が追加されても、UI から選択/アップロードで可視化できる柔軟性を保つ。

## 主要ファイル
- `streamlit_app.py`: Streamlit アプリ本体。CSV 読込、列正規化、構造検出、Plotly での描画を提供。
- `pyproject.toml`: uv が参照する依存定義と開発用グループ（dev）。
- `uv.lock`: ロックファイル（コミット対象）。
- `myproject.toml`: 人間向けの運用メモ（uv は直接は参照しない）。

## 実行方法（ローカル / CI）
- 依存同期: `uv sync`
- 起動（通常）: `uv run -m streamlit run streamlit_app.py`
- 起動（ヘッドレス）: `uv run -m streamlit run streamlit_app.py --server.address 0.0.0.0 --server.port 8501 --server.headless true`
- Codex/サンドボックス環境ではソケット制限に注意。必要なら昇格実行で待受け可。

## 対応するデータ形式
- Long: `format` 列があり、値に JPG/JPEG/PNG/WEBP を含む。
- Wide: 列名に `jpg|jpeg|png|webp` を含む複数の数値列を自動で縦持ち化（melt）。
- 不明時はサイドバーから手動指定できるようにする（本アプリは対応済み）。

## 必要な開発ツール（uv 管理 / 2025 年ベストプラクティス）
- 本番依存: `streamlit`, `pandas`, `plotly`, `pyarrow`
- 開発依存（dev グループ）:
  - `ruff`（lint, format）
  - `mypy`（型チェック）
  - `pytest`（テスト）
- 代表コマンド:
  - Lint+自動修正: `uv run ruff check --fix .`
  - フォーマット: `uv run ruff format .`
  - 型チェック: `uv run mypy .`
  - テスト: `uv run pytest -q`

## 依存の追加/更新
- 本番依存: `uv add <pkg>`
- 開発依存: `uv add --group dev <pkg>`
- アップグレード: `uv lock --upgrade` → `uv sync`

## コーディング/変更方針
- 配布用のビルド対応は不要（アプリ用途）。
- 既存のスタイル/構成を尊重し、必要最小限の変更に留める。
- 追加 CSV に備え、列名のばらつきやフォーマット名の同義（JPG/JPEG）を正規化する。
- 大きな変更前に README の手順が崩れないかを確認する。

## トラブルシューティング
- ポート競合: `--server.port` を変更。
- ソケット権限エラー: ローカル実行、または昇格実行を利用。
- CSV 解析失敗: サイドバーで区切り/エンコーディング/構造（Long/Wide）を手動指定。

