# Image Compression Benchmark - Makefile
#
# 使用方法:
#   make help          - ヘルプを表示
#   make render        - コンテナ内で Quarto レンダリング
#   make render-local  - ローカル環境で Quarto レンダリング
#   make clean         - 生成ファイルを削除

.PHONY: help render render-local container-build clean preview

CONTAINER_ENGINE ?= podman
IMAGE_NAME := quarto-build

help:
	@echo "利用可能なコマンド:"
	@echo "  make render         - コンテナ内で Quarto レンダリング (推奨)"
	@echo "  make render-local   - ローカル環境で Quarto レンダリング"
	@echo "  make preview        - ローカルプレビューサーバー起動"
	@echo "  make container-build - コンテナイメージをビルド"
	@echo "  make clean          - 生成ファイルを削除"

# コンテナイメージをビルド
container-build:
	$(CONTAINER_ENGINE) build -t $(IMAGE_NAME) -f Containerfile .

# コンテナ内でレンダリング
render: container-build
	$(CONTAINER_ENGINE) run --rm \
		-v $(PWD):/workspace:Z \
		-w /workspace \
		$(IMAGE_NAME)

# ローカル環境でレンダリング
render-local:
	uv sync
	cd docs && quarto render

# ローカルプレビュー
preview:
	uv sync
	cd docs && quarto preview

# 生成ファイルを削除
clean:
	rm -rf docs/_site
	rm -rf docs/.quarto
	rm -rf docs/_freeze

