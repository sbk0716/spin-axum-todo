#!/bin/bash
# =============================================================================
# setup.sh: 初期セットアップスクリプト
# =============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "=== spin-axum-todo セットアップ ==="

# .env ファイルの作成
if [ ! -f "$PROJECT_ROOT/.env" ]; then
    echo ">>> .env ファイルを作成中..."
    cp "$PROJECT_ROOT/.env.example" "$PROJECT_ROOT/.env"
    echo "    .env ファイルを作成しました"
else
    echo "    .env ファイルは既に存在します"
fi

# Docker Compose でインフラを起動
echo ">>> Docker Compose でインフラを起動中..."
cd "$PROJECT_ROOT"
docker compose up -d

# PostgreSQL の起動を待機
echo ">>> PostgreSQL の起動を待機中..."
until docker compose exec -T postgres pg_isready -U app -d app > /dev/null 2>&1; do
    sleep 1
done
echo "    PostgreSQL が起動しました"

# Redis の起動を待機
echo ">>> Redis の起動を待機中..."
until docker compose exec -T redis redis-cli ping > /dev/null 2>&1; do
    sleep 1
done
echo "    Redis が起動しました"

# sqlx-cli のインストール確認
if ! command -v sqlx &> /dev/null; then
    echo ">>> sqlx-cli をインストール中..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# .env から環境変数を読み込み
if [ -f "$PROJECT_ROOT/.env" ]; then
    export $(cat "$PROJECT_ROOT/.env" | grep -v '^#' | xargs)
fi

# マイグレーションの実行
echo ">>> マイグレーションを実行中..."
cd "$PROJECT_ROOT/core/api"
DATABASE_URL="${DATABASE_WRITER_URL:-postgres://app:app@localhost:5432/app}" sqlx migrate run
echo "    マイグレーションが完了しました"

# Spin CLI のインストール確認
if ! command -v spin &> /dev/null; then
    echo ""
    echo "⚠️  Spin CLI がインストールされていません"
    echo "    以下のいずれかの方法でインストールしてください:"
    echo ""
    echo "    # Homebrew（macOS）"
    echo "    brew tap spinframework/tap"
    echo "    brew install spinframework/tap/spin"
    echo ""
    echo "    # または公式インストーラ（macOS/Linux）"
    echo "    curl -fsSL https://spinframework.dev/downloads/install.sh | bash"
fi

echo ""
echo "=== セットアップ完了 ==="
echo ""
echo "次のステップ:"
echo "  1. ./scripts/dev.sh を実行して開発環境を起動"
echo "  2. または以下を手動で実行:"
echo "     - Core 層: cd core && cargo run -p api"
echo "     - Edge 層: cd edge && spin build && spin up"
