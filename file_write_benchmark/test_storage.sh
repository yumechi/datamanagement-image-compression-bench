#!/bin/bash

# ファイル書き込みとストレージ容量確認スクリプト
# 使用方法: ./test_storage.sh <テストディレクトリパス>

set -e  # エラー時に終了

# 引数チェック
if [ $# -lt 1 ]; then
    echo "使用方法: $0 <テストディレクトリパス> [ファイル数]"
    echo "例: $0 /tmp/storage_test"
    echo "例: $0 /tmp/storage_test 10000"
    exit 1
fi

TEST_DIR="$1"
FILE_COUNT="${2:-10000}"  # デフォルト10,000ファイル

echo "========================================="
echo "ストレージ容量テスト"
echo "========================================="
echo "テストディレクトリ: $TEST_DIR"
echo "作成ファイル数: $FILE_COUNT"
echo ""

# テストディレクトリの親ディレクトリが存在するか確認
PARENT_DIR=$(dirname "$TEST_DIR")
if [ ! -d "$PARENT_DIR" ]; then
    echo "エラー: 親ディレクトリ '$PARENT_DIR' が存在しません"
    exit 1
fi

# 既存のテストディレクトリを削除（存在する場合）
if [ -d "$TEST_DIR" ]; then
    echo "既存のテストディレクトリを削除中..."
    rm -rf "$TEST_DIR"
fi

# テストディレクトリを作成
echo "テストディレクトリを作成中..."
mkdir -p "$TEST_DIR"

# 書き込み前のストレージ容量を確認
echo ""
echo "=== 書き込み前のストレージ容量 ==="
df -h "$TEST_DIR"
echo ""

# ファイル書き込み開始
echo "ファイル書き込み開始..."
START_TIME=$(date +%s.%N)

for i in $(seq 0 $((FILE_COUNT - 1))); do
    # 10KBのファイルを作成
    head -c 10240 /dev/zero > "$TEST_DIR/file_$(printf '%08d' $i).dat"

    # 進捗表示（1000ファイルごと）
    if [ $((i % 1000)) -eq 0 ] && [ $i -ne 0 ]; then
        echo "  $i ファイル作成完了..."
    fi
done

END_TIME=$(date +%s.%N)
ELAPSED=$(echo "$END_TIME - $START_TIME" | bc)

echo "ファイル書き込み完了: $FILE_COUNT ファイル"
echo "処理時間: ${ELAPSED} 秒"
echo ""

# 書き込み後のストレージ容量を確認
echo "=== 書き込み後のストレージ容量 ==="
df -h "$TEST_DIR"
echo ""

# テストディレクトリのサイズを確認
echo "=== テストディレクトリのサイズ ==="
du -sh "$TEST_DIR"
echo ""

# クリーンアップ確認
read -p "テストデータを削除しますか？ (y/N): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "テストデータを削除中..."
    rm -rf "$TEST_DIR"
    echo "削除完了"

    # 削除後のストレージ容量を確認
    echo ""
    echo "=== 削除後のストレージ容量 ==="
    df -h "$PARENT_DIR"
else
    echo "テストデータは保持されます: $TEST_DIR"
fi

echo ""
echo "========================================="
echo "テスト完了"
echo "========================================="
