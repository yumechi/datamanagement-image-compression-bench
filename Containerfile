# Quarto ドキュメントビルド用コンテナ
# 
# 使用方法:
#   podman build -t quarto-build -f Containerfile .
#   podman run --rm -v $(pwd):/workspace quarto-build
#
# または make コマンド:
#   make container-build
#   make render

FROM python:3.13-slim

# 必要なシステムパッケージ
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Quarto をインストール
ARG QUARTO_VERSION=1.7.31
RUN curl -LO https://github.com/quarto-dev/quarto-cli/releases/download/v${QUARTO_VERSION}/quarto-${QUARTO_VERSION}-linux-amd64.tar.gz \
    && tar -xzf quarto-${QUARTO_VERSION}-linux-amd64.tar.gz \
    && mv quarto-${QUARTO_VERSION} /opt/quarto \
    && ln -s /opt/quarto/bin/quarto /usr/local/bin/quarto \
    && rm quarto-${QUARTO_VERSION}-linux-amd64.tar.gz

# uv をインストール
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin:$PATH"

WORKDIR /workspace

# デフォルトコマンド: 依存同期 + レンダリング
CMD ["sh", "-c", "uv sync && quarto render docs/"]

