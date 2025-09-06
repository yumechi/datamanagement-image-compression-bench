# Image Format Results Viewer (Streamlit + uv)

本プロジェクトは、画像フォーマット（JPG/PNG/WEBP など）の比較結果 CSV を読み込み、Streamlit で可視化するためのアプリです。Python は uv で管理し、配布用パッケージ化は行いません（アプリ用途）。

## 管理方針（2025 ベストプラクティス）
- 依存・環境管理は uv を使用
  - 依存宣言: `pyproject.toml`
  - ロックファイル: `uv.lock`（コミット推奨）
  - 仮想環境: `.venv`（リポジトリローカル）
- ドキュメント的な補助ファイル
  - `myproject.toml`: チーム向けの人間可読メモ（スクリプト例や依存の意図を記載）。uv は直接は参照しません。
- ビルド/配布は対象外
  - 配布用の build/metadata は不要。アプリとして実行できれば OK。

## セットアップ
- 依存の同期（初回/変更時）
  - `uv sync`
- Python の実行（以降は `.venv` を意識せず `uv run ...` を利用）

## 実行方法（Streamlit）
- 通常起動
  - `uv run -m streamlit run streamlit_app.py`
- ヘッドレス/ポート指定
  - `uv run -m streamlit run streamlit_app.py --server.address 0.0.0.0 --server.port 8501 --server.headless true`
- 停止/ログ
  - 起動をバックグラウンドにする場合はシェル機能を利用（例: `... > streamlit.log 2>&1 &` / `tail -f streamlit.log` / `kill <PID>`）

## CSV の扱い
- `result_csv/` 配下の `*.csv` を再帰的に自動検出（`*format*results*.csv` を優先）。UI からのアップロードも可。
- データ構造の自動判定
  - Long 形式: `format` 列がある（JPG/PNG/WEBP 等）
  - Wide 形式: 列名に `jpg|jpeg|png|webp` を含む複数列（例: `psnr_jpg`, `psnr_png`）→ 自動で縦持ち化
  - 不明な場合はサイドバーから手動指定可
- X/Y 軸の数値列を選択、フォーマットフィルタ、ライン/散布切替、同一 X での平均集計などに対応

## 依存の追加・削除・更新
- 本番依存の追加
  - `uv add <pkg>`
- 開発依存の追加（lint/型/テスト等）
  - `uv add --group dev <pkg>`
- 依存の削除
  - `uv remove <pkg>`
- アップグレード（全体）
  - `uv lock --upgrade`
- 反映（.venv 更新）
  - `uv sync`

## 開発ツール
- Lint & フォーマット
  - `uv run ruff check --fix .`
  - `uv run ruff format .`
- 型チェック
  - `uv run mypy .`
- テスト
  - `uv run pytest -q`

## 重要ファイル
- `streamlit_app.py`: Streamlit アプリ本体
- `pyproject.toml`: 依存と開発グループの定義（uv が参照）
- `uv.lock`: ロックファイル（必ずコミット）
- `myproject.toml`: チーム向けのメモ（補助的）
- `.gitignore`: Python/uv/Streamlit ログ等を除外
- `image_comparison_format_results.csv`: 例示的な入力 CSV（将来的に別 CSV の追加も想定）

## トラブルシューティング
- ポート競合: `--server.port` で変更
- 外部アクセス: `--server.address 0.0.0.0` を指定
- 権限/サンドボックスで待受け不可な場合
  - ローカル環境での実行（推奨）: `uv run -m streamlit run streamlit_app.py`
  - もしくは対話型 HTML の静的出力スクリプトを追加して回避（必要なら要望ください）

## ライセンス/配布
- 配布目的ではありません。社内/個人用途の可視化ツールとして利用してください。
