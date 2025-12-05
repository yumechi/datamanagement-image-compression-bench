# プロジェクト設定

## コミュニケーション
- **言語**: 日本語
- このプロジェクトでの全てのコミュニケーションは日本語で行ってください

## プロジェクト概要
- **目的**: 「datamanagement」という本のためのコード管理
- **対象領域**: データ管理技術の比較検討とベンチマーク
- **主要な研究テーマ**:
  - 画像処理と圧縮技術の比較
  - ファイルシステムI/O性能評価
  - データ可視化（Quarto + GitHub Pages）

## プロジェクト構成

### docs/ (可視化)
- Quarto ベースのベンチマーク結果可視化
- GitHub Pages で公開
- 詳細は `AGENTS.md` を参照

### compression_format_comparison/
- Rust実装の圧縮フォーマットベンチマークツール
- 詳細は `compression_format_comparison/CLAUDE.md` を参照

### image_format_comparison/
- Rust実装の画像フォーマット比較ベンチマークツール
- 詳細は `image_format_comparison/CLAUDE.md` を参照

### image_quality_comparison/
- Rust実装の画像品質比較ベンチマークツール
- 詳細は `image_quality_comparison/CLAUDE.md` を参照

### file_write_benchmark/
- Rust実装のファイル書き込みベンチマークツール
- 詳細は `file_write_benchmark/CLAUDE.md` を参照

### result_csv/
- ベンチマーク結果の CSV ファイル
- 可視化ドキュメント (docs/) から参照される

## 開発環境
- **プラットフォーム**: Linux (WSL2)
- **言語**: Rust (ベンチマーク), Python (可視化)
- **可視化**: Quarto + matplotlib
- **パッケージ管理**: uv (Python), Cargo (Rust)
- **コンテナ**: Podman / Docker

## 全体的なガイドライン

### コーディング規約
- 全てのコード、コメント、ドキュメントは日本語で記述
- 各言語のベストプラクティスに従う
- パフォーマンスと可読性のバランスを考慮

### ドキュメント管理
- 各サブプロジェクトは独自のCLAUDE.mdで仕様を管理
- ルートのCLAUDE.mdはプロジェクト全体の基本方針のみ記載
- README.mdは各サブプロジェクトで詳細な使用方法を記載
- AGENTS.md は AI エージェント向けの作業指針

### データ管理
- ベンチマーク結果はCSV形式で `result_csv/` に保存
- 生成データは自動クリーンアップを推奨
- バージョン管理対象はソースコードと設定ファイルのみ

### 可視化のビルド
```bash
# コンテナ使用（推奨）
make render

# Docker の場合
CONTAINER_ENGINE=docker make render
```
