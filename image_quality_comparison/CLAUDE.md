# image_quality_comparison - 画像品質比較ベンチマーク

## プロジェクト概要

このディレクトリは「datamanagement」本のための**画像品質比較ベンチマークツール**です。
WebP画像の品質設定による圧縮効果とファイルサイズの関係を定量的に分析し、品質と容量のトレードオフを検証します。

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
  - ランダムノイズPNG画像生成
  - PNG → WebP品質別変換

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
cd image_quality_comparison
cargo build --release
```

### 実行方法

#### メインプログラム（image_quality_comparison）
```bash
# デフォルト実行（100枚の画像、10ラウンド、6品質レベル）
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
# 軽量テスト実行（デフォルト: 5枚、1ラウンド、3品質レベル）
cargo run --bin test_program

# カスタムテスト
cargo run --bin test_program -- [画像枚数] [ラウンド数]

# 例: 10枚の画像、2ラウンド
cargo run --bin test_program -- 10 2
```

## プログラム構成

### バイナリ

1. **image_quality_comparison** (`src/main.rs`)
   - フルスケールベンチマーク
   - デフォルト: 100枚、10ラウンド
   - 品質レベル: 100%, 90%, 80%, 70%, 60%, 50%（6段階）
   - 出力: `image_quality_comparison_results.csv`

2. **test_program** (`src/test.rs`)
   - 軽量テスト用
   - デフォルト: 5枚、1ラウンド
   - 品質レベル: 100%, 80%, 60%（3段階）
   - 出力: `image_quality_comparison_test_results.csv`

## 処理フロー

1. **ランダム画像生成**: ImageMagickで1024x1024のランダムノイズPNG画像を生成（4並列処理）
2. **品質別WebP変換**: PNG画像を複数の品質レベルでWebPに変換（4並列処理）
3. **統計計算**: 各品質レベルでのファイルサイズ統計と圧縮率を算出
4. **CSV出力**: 指定されたラウンド数の実行結果をCSVファイルに出力
5. **クリーンアップ**: 各ラウンド後に画像ファイルを自動削除（ディスク容量節約）

## 品質レベル設定

### メインプログラム（6段階）
```rust
let quality_levels = [100, 90, 80, 70, 60, 50];
```

| 品質レベル | 用途 |
|----------|------|
| 100% | 最高品質（ベースライン） |
| 90% | 高品質（視覚的劣化ほぼなし） |
| 80% | 標準品質（一般的な使用） |
| 70% | 中品質（容量重視） |
| 60% | 低品質（高圧縮） |
| 50% | 最低品質（最大圧縮） |

### テストプログラム（3段階）
```rust
let quality_levels = [100, 80, 60];
```

高・中・低の代表的な品質レベルのみをテストします。

## WebP変換コマンド

### 品質別変換
```bash
# 品質100%（最高品質）
convert input.png -quality 100 output_q100.webp

# 品質90%
convert input.png -quality 90 output_q90.webp

# 品質80%
convert input.png -quality 80 output_q80.webp

# 品質70%
convert input.png -quality 70 output_q70.webp

# 品質60%
convert input.png -quality 60 output_q60.webp

# 品質50%（最大圧縮）
convert input.png -quality 50 output_q50.webp
```

## 出力データ形式

### CSVカラム
- **run_number**: 実行回数（ラウンド番号）
- **quality**: WebP品質レベル（100, 90, 80, 70, 60, 50）
- **total_size**: 指定枚数の画像の合計ファイルサイズ（バイト）
- **average_size**: 1枚あたりの平均ファイルサイズ（バイト）
- **min_size**: 最も小さいファイルサイズ（バイト）
- **max_size**: 最も大きいファイルサイズ（バイト）
- **median_size**: ファイルサイズの中央値（バイト）
- **compression_ratio**: PNG基準での圧縮率（WebPサイズ / PNGサイズ）

### 圧縮率の計算
```rust
compression_ratio = webp_average_size / png_average_size
```
- **値が小さい**: 高圧縮（容量削減効果が大きい）
- **値が大きい**: 低圧縮（元のサイズに近い）

### 出力ファイル
- `image_quality_comparison_results.csv`: メインプログラムの結果
- `image_quality_comparison_test_results.csv`: テストプログラムの結果

### 一時ディレクトリ
- `images_run_N/`: 各ラウンドごとの画像ファイル格納（統計取得後に自動削除）

## 実行時間の目安

### 標準設定（100枚×10ラウンド×6品質）
- **画像生成**: 約1-2分/ラウンド
- **品質別WebP変換**: 約2-4分/ラウンド（6品質レベル）
- **合計**: 約30-60分

### テスト設定（5枚×1ラウンド×3品質）
- **合計**: 約20-40秒

## パフォーマンス特性

### 並列処理
- **並列度**: 4並列（画像生成と変換の両方）
- **非同期ランタイム**: Tokio
- **タスク管理**: `JoinSet`を使用した並行実行

### 画像仕様
- **サイズ**: 1024x1024ピクセル
- **内容**: ランダムノイズ（plasma:fractal）
- **ベースライン形式**: PNG（可逆圧縮）
- **変換対象**: WebP（品質別）

### ImageMagick変換コマンド実装例

#### PNG生成
```bash
convert -size 1024x1024 plasma:fractal output.png
```

#### WebP変換（品質指定）
```bash
convert input.png -quality {quality} output_q{quality}.webp
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

// 品質レベルごとに4並列で変換
for &quality in &quality_levels {
    let mut join_set = JoinSet::new();
    for i in 0..4 {
        join_set.spawn(async move {
            // WebP変換処理
        });
    }

    while let Some(result) = join_set.join_next().await {
        result??;
    }
}
```

## 期待される結果

### 品質とファイルサイズの関係
一般的な傾向（ランダムノイズ画像の場合）：
- **100%**: PNG比で70-80%程度のサイズ
- **90%**: PNG比で50-60%程度のサイズ
- **80%**: PNG比で40-50%程度のサイズ
- **70%**: PNG比で30-40%程度のサイズ
- **60%**: PNG比で25-35%程度のサイズ
- **50%**: PNG比で20-30%程度のサイズ

### 品質レベルごとの特徴
| 品質 | 圧縮率 | 視覚的品質 | 推奨用途 |
|------|--------|-----------|---------|
| 100% | 低 | 最高 | アーカイブ、医療画像 |
| 90% | 中低 | 非常に高い | プロフェッショナル写真 |
| 80% | 中 | 高い | Web標準、一般的な写真 |
| 70% | 中高 | 良好 | SNS投稿、プレビュー |
| 60% | 高 | 許容範囲 | サムネイル、モバイル |
| 50% | 非常に高い | 低い | 極小サムネイル |

## プロジェクト目的

本ベンチマークツールは以下の判断材料を提供します：

- **品質と容量のトレードオフ**: 各品質レベルでの圧縮率の定量的測定
- **最適品質レベルの選択**: 用途に応じた品質設定の指針
- **統計的信頼性**: 複数ラウンド実行による性能の一貫性評価
- **データ保存戦略**: ストレージコスト削減のための最適化指針

### ユースケース
- Webサイトでの画像最適化（品質と読み込み速度のバランス）
- クラウドストレージコストの削減
- CDN転送量の削減
- モバイルアプリでのデータ使用量削減
- アーカイブシステムでのストレージ容量最適化

### 書籍「datamanagement」での活用
- 画像データ管理における品質設定の重要性
- 圧縮率と品質の定量的関係の可視化
- データ容量削減戦略の実践的な指針
