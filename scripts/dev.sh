#!/bin/bash
# =============================================================================
# dev.sh: 開発環境起動スクリプト
# =============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "=== spin-axum-todo 開発環境 ==="

# .env の読み込み
if [ -f "$PROJECT_ROOT/.env" ]; then
    export $(cat "$PROJECT_ROOT/.env" | grep -v '^#' | xargs)
fi

# Docker Compose でインフラを起動
echo ">>> インフラを起動中..."
cd "$PROJECT_ROOT"
docker compose up -d

# PostgreSQL の起動を待機
echo ">>> PostgreSQL の起動を待機中..."
until docker compose exec -T postgres pg_isready -U app -d app > /dev/null 2>&1; do
    sleep 1
done

# Redis の起動を待機
echo ">>> Redis の起動を待機中..."
until docker compose exec -T redis redis-cli ping > /dev/null 2>&1; do
    sleep 1
done

echo ""
echo "=== インフラ起動完了 ==="
echo ""
echo "以下のコマンドを別ターミナルで実行してください:"
echo ""
echo "  [Core 層]"
echo "    cd $PROJECT_ROOT/core && \\"
echo "      APP_ADDR=0.0.0.0:3001 \\"
echo "      DATABASE_WRITER_URL=postgres://app:app@localhost:5432/app \\"
echo "      DATABASE_READER_URL=postgres://app:app@localhost:5432/app \\"
echo "      REDIS_URL=redis://localhost:6379 \\"
echo "      JWT_SECRET=super-secret-key \\"
echo "      EDGE_SECRET=super-secret-edge-key \\"
echo "      RUST_LOG=info \\"
echo "      cargo run -p api"
echo ""
echo "  [Edge 層]"
echo "    cd $PROJECT_ROOT/edge && spin build && spin up"
echo ""
echo "=== 認証フロー（ローカル認証）==="
echo ""
echo "  # 1. ユーザー登録"
echo '  curl -X POST http://localhost:3000/api/auth/register \'
echo '    -H "Content-Type: application/json" \'
echo '    -d '\''{"email": "test@example.com", "password": "password123"}'\'''
echo ""
echo "  # 2. ログイン（JWT 取得）"
echo '  TOKEN=$(curl -s -X POST http://localhost:3000/api/auth/login \'
echo '    -H "Content-Type: application/json" \'
echo '    -d '\''{"email": "test@example.com", "password": "password123"}'\'' | jq -r ".token")'
echo ""
echo "  # 3. TODO 一覧を取得"
echo '  curl -H "Authorization: Bearer $TOKEN" http://localhost:3000/api/todos'
echo ""
echo "  # 統合テストを実行"
echo "  ./scripts/test-edge.sh  # Edge 経由"
echo "  ./scripts/test-core.sh  # Core 単体"
echo ""
