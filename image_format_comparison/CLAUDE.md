# image_format_comparison - 画像フォーマット比較ベンチマーク

## プロジェクト概要

このディレクトリは「datamanagement」本のための**画像フォーマット比較ベンチマークツール**です。
異なる画像フォーマット（PNG、JPG、WebP）のファイルサイズ効率を定量的に測定・比較します。

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
- **ImageMagick**: `convert`コマンドを使用
  - ランダムノイズ画像生成
  - 画像フォーマット変換（PNG → JPG, WebP）

#### インストールコマンド
```bash
# Ubuntu/Debian
sudo apt-get install imagemagick

# macOS
brew install imagemagick
```

## ビルド・実行方法

### ビルド
```bash
cd image_format_comparison
cargo build --release
```

### 実行方法

#### メインプログラム（image_format_comparison）
```bash
# デフォルト実行（100枚の画像、10ラウンド）
cargo run

# カスタム設定
cargo run -- [画像枚数] [ラウンド数]

# 例: 50枚の画像、5ラウンド
cargo run -- 50 5

# 例: テスト用（2枚、1ラウンド）
cargo run -- 2 1
```

#### テストプログラム（test_program）
```bash
# 軽量テスト実行（デフォルト: 5枚、1ラウンド）
cargo run --bin test_program

# カスタムテスト
cargo run --bin test_program -- [画像枚数] [ラウンド数]

# 例: 10枚の画像、2ラウンド
cargo run --bin test_program -- 10 2
```

## プログラム構成

### バイナリ

1. **image_format_comparison** (`src/main.rs`)
   - フルスケールベンチマーク
   - デフォルト: 100枚、10ラウンド
   - 出力: `image_format_comparison_results.csv`

2. **test_program** (`src/test.rs`)
   - 軽量テスト用
   - デフォルト: 5枚、1ラウンド
   - 出力: `image_format_comparison_test_results.csv`

## 処理フロー

1. **ランダム画像生成**: ImageMagickで1024x1024のランダムノイズPNG画像を生成（4並列処理）
2. **フォーマット変換**: 生成されたPNG画像をJPGとWebPに劣化なし変換（quality=100、4並列処理）
3. **統計計算**: 各フォーマットのファイルサイズ統計を算出
4. **CSV出力**: 指定されたラウンド数の実行結果をCSVファイルに出力
5. **クリーンアップ**: 各ラウンド後に画像ファイルを自動削除（ディスク容量節約）

## 対応画像フォーマット

### 比較対象の3フォーマット
- **PNG**: 可逆圧縮、アルファチャンネル対応
- **JPG**: 非可逆圧縮（本ベンチマークではquality=100で実行）
- **WebP**: Googleの次世代画像フォーマット（可逆/非可逆両対応、本ベンチマークではquality=100）

### 変換パラメータ
全ての変換は**quality=100**（最高品質）で実行し、圧縮アルゴリズムの効率のみを評価します。

```bash
# JPG変換
convert input.png -quality 100 output.jpg

# WebP変換
convert input.png -quality 100 output.webp
```

## 出力データ形式

### CSVカラム
- **run_number**: 実行回数（ラウンド番号）
- **format**: 画像フォーマット（PNG / JPG / WEBP）
- **total_size**: 指定枚数の画像の合計ファイルサイズ（バイト）
- **average_size**: 1枚あたりの平均ファイルサイズ（バイト）
- **min_size**: 最も小さいファイルサイズ（バイト）
- **max_size**: 最も大きいファイルサイズ（バイト）
- **median_size**: ファイルサイズの中央値（バイト）

### 出力ファイル
- `image_format_comparison_results.csv`: メインプログラムの結果
- `image_format_comparison_test_results.csv`: テストプログラムの結果

### 一時ディレクトリ
- `images_run_N/`: 各ラウンドごとの画像ファイル格納（統計取得後に自動削除）

## 実行時間の目安

### 標準設定（100枚×10ラウンド）
- **画像生成**: 約1-2分/ラウンド
- **フォーマット変換**: 約1-2分/ラウンド
- **合計**: 約20-40分

### テスト設定（5枚×1ラウンド）
- **合計**: 約10-30秒

## パフォーマンス特性

### 並列処理
- **並列度**: 4並列（画像生成と変換の両方）
- **非同期ランタイム**: Tokio
- **タスク管理**: `JoinSet`を使用した並行実行

### 画像仕様
- **サイズ**: 1024x1024ピクセル
- **内容**: ランダムノイズ（plasma:fractal）
- **初期形式**: PNG（可逆圧縮）

### ImageMagick変換コマンド

#### PNG生成
```bash
convert -size 1024x1024 plasma:fractal output.png
```

#### JPG変換（最高品質）
```bash
convert input.png -quality 100 output.jpg
```

#### WebP変換（最高品質）
```bash
convert input.png -quality 100 output.webp
```

## 開発時の注意事項

### コーディング規約
- 全てのコメントは日本語で記述
- Rustのベストプラクティスに従う
- 非同期処理にはTokioを使用

### エラーハンドリング
- ImageMagick（`convert`コマンド）の実行結果は必ずチェック
- ファイルI/O操作は適切にエラーハンドリング
- ユーザーへのエラーメッセージは日本語で明確に

### テスト戦略
- 実装変更時は`test_program`で動作確認
- フルベンチマークは最終確認時のみ実行
- CI/CD環境では軽量テストのみ実行推奨

### 並列処理の実装パターン
```rust
use tokio::task::JoinSet;

// 4並列でタスクを実行
let mut join_set = JoinSet::new();
for i in 0..4 {
    join_set.spawn(async move {
        // 並列処理内容
    });
}

// 全タスクの完了を待つ
while let Some(result) = join_set.join_next().await {
    result??;
}
```

## 期待される結果

### ファイルサイズ効率
一般的な傾向（ランダムノイズ画像の場合）：
- **PNG**: 中程度のサイズ（可逆圧縮ベースライン）
- **JPG**: 圧縮効率はデータパターンに依存
- **WebP**: 最も効率的な圧縮（PNGより小さい傾向）

### 品質保証
- **quality=100設定**: 全てのフォーマットで最高品質を使用
- **劣化最小化**: 可能な限り元画像の品質を保持
- **公平な比較**: 同一の元画像から全フォーマットを生成

## プロジェクト目的

本ベンチマークツールは以下の判断材料を提供します：

- **圧縮効率**: どのフォーマットが最も小さなファイルサイズを実現するか
- **統計的安定性**: 複数ラウンド実行による性能の一貫性評価
- **フォーマット選択指針**: データ管理戦略における最適な画像フォーマット選択
- **定量的データ**: 書籍「datamanagement」における技術的根拠

### ユースケース
- Webサイトでの画像最適化
- アーカイブシステムでのストレージ容量削減
- 画像配信CDNでの転送量削減
- クラウドストレージコストの最適化
