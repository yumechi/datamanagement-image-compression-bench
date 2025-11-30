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
  - データ可視化ツール

## プロジェクト構成

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

### python_visualization/
- Pythonベースのデータ可視化ツール（Streamlit）
- 詳細は `python_visualization/CLAUDE.md` を参照（準備中）

## 開発環境
- **プラットフォーム**: Linux (WSL2)
- **言語**: プロジェクトごとに異なる（Rust, Python）

## 全体的なガイドライン

### コーディング規約
- 全てのコード、コメント、ドキュメントは日本語で記述
- 各言語のベストプラクティスに従う
- パフォーマンスと可読性のバランスを考慮

### ドキュメント管理
- 各サブプロジェクトは独自のCLAUDE.mdで仕様を管理
- ルートのCLAUDE.mdはプロジェクト全体の基本方針のみ記載
- README.mdは各サブプロジェクトで詳細な使用方法を記載

### データ管理
- ベンチマーク結果はCSV形式で保存
- 生成データは自動クリーンアップを推奨
- バージョン管理対象はソースコードと設定ファイルのみ