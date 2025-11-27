# compression_format_comparison - 圧縮フォーマット比較ベンチマーク

## プロジェクト概要

このディレクトリは「datamanagement」本のための**圧縮フォーマット比較ベンチマークツール**です。
画像ファイルを対象に、複数の圧縮形式での圧縮効率と処理速度を定量的に測定します。

## 技術スタック

### 言語・フレームワーク
- **言語**: Rust
- **Edition**: 2024
- **非同期ランタイム**: Tokio 1.0 (full features)

### 主要依存関係
```toml
csv = "1.3"           # CSV出力用
serde = "1.0"         # データシリアライゼーション（derive feature有効）
rand = "0.8"          # ランダム値生成
tokio = "1.0"         # 非同期処理（full features）
```

### 外部コマンド依存
以下のシステムコマンドが必須：
- **ImageMagick**: ベンチマーク用画像生成
- **zip**: ZIP形式圧縮
- **tar**: TAR.GZ形式アーカイブ作成
- **zstd**: Zstandard圧縮
- **xz-utils**: XZ形式圧縮
- **p7zip-full**: 7-Zip形式圧縮

#### インストールコマンド
```bash
# Ubuntu/Debian
sudo apt-get install imagemagick zip tar zstd xz-utils p7zip-full

# macOS
brew install imagemagick zstd xz p7zip
```

## ビルド・実行方法

### ビルド
```bash
cd compression_format_comparison
cargo build --release
```

### 実行方法

#### メインプログラム（compression_format_comparison）
```bash
# デフォルト実行（100枚の画像、各形式100回圧縮）
cargo run

# カスタム設定
cargo run -- [画像枚数] [圧縮回数]

# 例: 50枚の画像、各形式50回圧縮
cargo run -- 50 50

# 例: テスト用（10枚、5回）
cargo run -- 10 5
```

#### テストプログラム（test_program）
```bash
# 軽量テスト実行（デフォルト: 10枚、3回圧縮、3形式のみ）
cargo run --bin test_program

# カスタムテスト
cargo run --bin test_program -- [画像枚数] [圧縮回数]

# 例: 20枚の画像、5回圧縮
cargo run --bin test_program -- 20 5
```

## プログラム構成

### バイナリ

1. **compression_format_comparison** (`src/main.rs`)
   - フルスケールベンチマーク
   - 5つの圧縮形式をテスト（ZIP, TAR.GZ, ZSTD, XZ, 7Z）
   - 出力: `compression_format_comparison_results.csv`

2. **test_program** (`src/test.rs`)
   - 軽量テスト用
   - 3つの圧縮形式のみ（ZIP, TAR.GZ, ZSTD）
   - 出力: `compression_format_comparison_test_results.csv`

## 処理フロー

1. **画像生成**: ImageMagickで1024x1024のランダムノイズPNG画像を生成（4並列処理）
2. **ディレクトリ準備**: `benchmark_images/`に画像を配置
3. **圧縮ベンチマーク**: 各圧縮形式で指定回数の圧縮を実行
4. **統計測定**: 圧縮率、処理時間、圧縮速度を記録
5. **CSV出力**: 結果をCSVファイルに保存
6. **クリーンアップ**: 実験用ファイルを自動削除

## 対応圧縮フォーマット

### メインプログラム（5形式）
- **ZIP**: 汎用アーカイブ形式
- **TAR.GZ**: UNIX標準のgzip圧縮tar
- **ZSTD**: Facebook開発の高性能圧縮
- **XZ**: 高圧縮率のLZMA2ベース
- **7Z**: 7-Zipの高効率圧縮形式

### テストプログラム（3形式）
- ZIP
- TAR.GZ
- ZSTD

## 出力データ形式

### CSVカラム
- **run**: 実行回数
- **format**: 圧縮形式名
- **original_size**: 元ファイルサイズ（バイト）
- **compressed_size**: 圧縮後サイズ（バイト）
- **compression_ratio**: 圧縮率（圧縮後/元サイズ）
- **compression_time_ms**: 圧縮時間（ミリ秒）
- **compression_speed_mbps**: 圧縮速度（MB/秒）

### 出力ファイル
- `compression_format_comparison_results.csv`: メインプログラムの結果
- `compression_format_comparison_test_results.csv`: テストプログラムの結果

## 実行時間の目安

### 標準設定（100枚×100回×5形式）
- **画像生成**: 約1-2分
- **圧縮テスト**: 約10-30分（システム性能による）
- **合計**: 約30-45分

### テスト設定（10枚×3回×3形式）
- **合計**: 約2-5分

## パフォーマンス特性

### 画像生成
- **並列度**: 4並列
- **画像サイズ**: 1024x1024ピクセル
- **形式**: PNG（ランダムノイズ）

### 圧縮ベンチマーク
- **順次実行**: 各圧縮は順番に実行（正確な時間測定のため）
- **クリーンアップ**: 各圧縮後にファイルを削除（ディスク容量節約）

## 開発時の注意事項

### コーディング規約
- 全てのコメントは日本語で記述
- Rustのベストプラクティスに従う
- 非同期処理にはTokioを使用

### エラーハンドリング
- 外部コマンドの実行結果は必ずチェック
- ファイルI/O操作は適切にエラーハンドリング
- ユーザーへのエラーメッセージは日本語で明確に

### テスト戦略
- 実装変更時は`test_program`で動作確認
- フルベンチマークは最終確認時のみ実行
- CI/CD環境では軽量テストのみ実行推奨

## 期待される結果

### 圧縮効率
- **XZ**: 最高レベルの圧縮率（処理時間は長め）
- **7Z**: 高い圧縮率と多機能性
- **ZSTD**: 高速処理と優れた圧縮率のバランス
- **TAR.GZ**: 標準的な圧縮性能
- **ZIP**: バランスの良い圧縮率と処理速度

### 処理速度
- **ZSTD**: 最高速クラスの圧縮速度
- **ZIP**: 高速処理
- **TAR.GZ**: 中程度の速度
- **7Z**: 圧縮率重視のため低速
- **XZ**: 最も低速だが最高圧縮率

## プロジェクト目的

本ベンチマークツールは以下の判断材料を提供します：

- **圧縮効率**: どの形式が最も小さなファイルサイズを実現するか
- **処理速度**: どの形式が最も高速に処理できるか
- **実用性評価**: 圧縮率と速度のバランスからの最適形式選択
- **統計的信頼性**: 複数回実行による性能の安定性評価

データ管理戦略において、用途に応じた最適な圧縮形式の選択指針を提供します。