#!/bin/bash
# =============================================================================
# test-edge.sh: Edge 層経由の統合テスト
# =============================================================================
# Edge 層（JWT 認証）経由で全 API エンドポイントをテストする。
#
# 前提条件:
#   - Core 層が localhost:3001 で起動していること
#   - Edge 層が localhost:3000 で起動していること
#
# 使用方法:
#   ./scripts/test-edge.sh
# =============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

EDGE_URL="http://localhost:3000"
CORE_URL="http://localhost:3001"

# カウンター
PASSED=0
FAILED=0

# テスト結果を記録
pass() {
    echo "    ✅ $1"
    PASSED=$((PASSED + 1))
}

fail() {
    echo "    ❌ $1"
    FAILED=$((FAILED + 1))
}

# ユニークなテストユーザーを生成
TEST_EMAIL="test-$(date +%s)@example.com"
TEST_PASSWORD="password123"

echo "=== spin-axum-todo Edge 層統合テスト ==="
echo "    テストユーザー: $TEST_EMAIL"
echo "    Edge URL: $EDGE_URL"
echo "    Core URL: $CORE_URL"

# =============================================================================
# ヘルスチェック
# =============================================================================
echo ""
echo "=== ヘルスチェック ==="

echo ">>> Core 層 Health check..."
HEALTH=$(curl -s "$CORE_URL/health" || echo "FAILED")
if echo "$HEALTH" | grep -q "ok"; then
    pass "Core 層 Health check"
else
    fail "Core 層 Health check: $HEALTH"
    echo "    Core 層が起動していない可能性があります"
    exit 1
fi

echo ">>> Edge 層 Health check..."
HEALTH=$(curl -s "$EDGE_URL/health" || echo "FAILED")
if echo "$HEALTH" | grep -q "ok"; then
    pass "Edge 層 Health check"
else
    fail "Edge 層 Health check: $HEALTH"
    echo "    Edge 層が起動していない可能性があります"
    exit 1
fi

# =============================================================================
# 認証 API テスト
# =============================================================================
echo ""
echo "=== 認証 API テスト ==="

echo ">>> POST /api/auth/register - ユーザー登録..."
REGISTER_RESPONSE=$(curl -s -X POST "$EDGE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\", \"display_name\": \"Test User\"}")
if echo "$REGISTER_RESPONSE" | grep -q '"id"'; then
    USER_ID=$(echo "$REGISTER_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
    pass "ユーザー登録成功: $USER_ID"
else
    fail "ユーザー登録失敗: $REGISTER_RESPONSE"
    exit 1
fi

echo ">>> POST /api/auth/register - 重複メールアドレス（409 期待）..."
DUPLICATE_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}")
if [ "$DUPLICATE_RESPONSE" = "409" ]; then
    pass "重複メールアドレスで 409"
else
    fail "期待: 409, 実際: $DUPLICATE_RESPONSE"
fi

echo ">>> POST /api/auth/register - 無効なメール形式（400 期待）..."
INVALID_EMAIL=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d '{"email": "invalid-email", "password": "password123"}')
if [ "$INVALID_EMAIL" = "400" ]; then
    pass "無効なメール形式で 400"
else
    fail "期待: 400, 実際: $INVALID_EMAIL"
fi

echo ">>> POST /api/auth/register - 短いパスワード（400 期待）..."
SHORT_PASSWORD=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d '{"email": "short@example.com", "password": "short"}')
if [ "$SHORT_PASSWORD" = "400" ]; then
    pass "短いパスワードで 400"
else
    fail "期待: 400, 実際: $SHORT_PASSWORD"
fi

echo ">>> POST /api/auth/login - ログイン..."
LOGIN_RESPONSE=$(curl -s -X POST "$EDGE_URL/api/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}")
if echo "$LOGIN_RESPONSE" | grep -q '"token"'; then
    TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
    pass "ログイン成功"
else
    fail "ログイン失敗: $LOGIN_RESPONSE"
    exit 1
fi

echo ">>> POST /api/auth/login - 誤ったパスワード（401 期待）..."
WRONG_PASSWORD=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"wrongpassword\"}")
if [ "$WRONG_PASSWORD" = "401" ]; then
    pass "誤ったパスワードで 401"
else
    fail "期待: 401, 実際: $WRONG_PASSWORD"
fi

echo ">>> POST /api/auth/login - 存在しないユーザー（401 期待）..."
NO_USER=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/auth/login" \
    -H "Content-Type: application/json" \
    -d '{"email": "nonexistent@example.com", "password": "password123"}')
if [ "$NO_USER" = "401" ]; then
    pass "存在しないユーザーで 401"
else
    fail "期待: 401, 実際: $NO_USER"
fi

# =============================================================================
# 認証チェックテスト
# =============================================================================
echo ""
echo "=== 認証チェックテスト ==="

echo ">>> GET /api/todos - 認証なし（401 期待）..."
NO_AUTH=$(curl -s -o /dev/null -w "%{http_code}" "$EDGE_URL/api/todos")
if [ "$NO_AUTH" = "401" ]; then
    pass "認証なしで 401"
else
    fail "期待: 401, 実際: $NO_AUTH"
fi

echo ">>> GET /api/todos - 無効なトークン（401 期待）..."
INVALID_TOKEN=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "Authorization: Bearer invalid-token" "$EDGE_URL/api/todos")
if [ "$INVALID_TOKEN" = "401" ]; then
    pass "無効なトークンで 401"
else
    fail "期待: 401, 実際: $INVALID_TOKEN"
fi

# =============================================================================
# TODO CRUD テスト
# =============================================================================
echo ""
echo "=== TODO CRUD テスト ==="

echo ">>> GET /api/todos - 一覧取得（空）..."
TODOS=$(curl -s -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos")
if echo "$TODOS" | grep -q "\[\]"; then
    pass "空の TODO 一覧を取得"
else
    pass "TODO 一覧を取得"
fi

echo ">>> POST /api/todos - TODO 作成..."
CREATE_RESPONSE=$(curl -s -X POST "$EDGE_URL/api/todos" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"title": "テスト TODO", "description": "統合テストで作成"}')
TODO_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
if [ -n "$TODO_ID" ]; then
    pass "TODO 作成: $TODO_ID"
else
    fail "TODO 作成失敗: $CREATE_RESPONSE"
    exit 1
fi

echo ">>> POST /api/todos - 空タイトル（400 期待）..."
EMPTY_TITLE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/todos" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"title": "", "description": "空タイトル"}')
if [ "$EMPTY_TITLE" = "400" ]; then
    pass "空タイトルで 400"
else
    fail "期待: 400, 実際: $EMPTY_TITLE"
fi

echo ">>> GET /api/todos/{id} - TODO 取得..."
GET_RESPONSE=$(curl -s -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos/$TODO_ID")
if echo "$GET_RESPONSE" | grep -q "$TODO_ID"; then
    pass "TODO 取得"
else
    fail "TODO 取得失敗: $GET_RESPONSE"
fi

echo ">>> GET /api/todos?completed=false - 未完了フィルタ..."
INCOMPLETE=$(curl -s -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos?completed=false")
if echo "$INCOMPLETE" | grep -q "$TODO_ID"; then
    pass "未完了フィルタで TODO を取得"
else
    fail "未完了フィルタで TODO が見つからない"
fi

echo ">>> PATCH /api/todos/{id} - タイトル更新..."
PATCH_TITLE=$(curl -s -X PATCH "$EDGE_URL/api/todos/$TODO_ID" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"title": "更新されたタイトル"}')
if echo "$PATCH_TITLE" | grep -q '更新されたタイトル'; then
    pass "タイトル更新"
else
    fail "タイトル更新失敗: $PATCH_TITLE"
fi

echo ">>> PATCH /api/todos/{id} - 完了に更新..."
PATCH_COMPLETE=$(curl -s -X PATCH "$EDGE_URL/api/todos/$TODO_ID" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"completed": true}')
if echo "$PATCH_COMPLETE" | grep -q '"completed":true'; then
    pass "完了に更新"
else
    fail "完了に更新失敗: $PATCH_COMPLETE"
fi

echo ">>> GET /api/todos?completed=true - 完了フィルタ..."
COMPLETE=$(curl -s -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos?completed=true")
if echo "$COMPLETE" | grep -q "$TODO_ID"; then
    pass "完了フィルタで TODO を取得"
else
    fail "完了フィルタで TODO が見つからない"
fi

echo ">>> GET /api/todos?completed=false - 完了 TODO が除外されることを確認..."
INCOMPLETE2=$(curl -s -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos?completed=false")
if echo "$INCOMPLETE2" | grep -q "$TODO_ID"; then
    fail "完了 TODO が未完了フィルタに含まれている"
else
    pass "完了 TODO が未完了フィルタから除外"
fi

echo ">>> DELETE /api/todos/{id} - TODO 削除..."
DELETE_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE \
    -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos/$TODO_ID")
if [ "$DELETE_RESPONSE" = "204" ]; then
    pass "TODO 削除"
else
    fail "TODO 削除失敗: HTTP $DELETE_RESPONSE"
fi

echo ">>> GET /api/todos/{id} - 削除後取得（404 期待）..."
DELETED=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos/$TODO_ID")
if [ "$DELETED" = "404" ]; then
    pass "削除後 404"
else
    fail "期待: 404, 実際: $DELETED"
fi

echo ">>> GET /api/todos/{id} - 存在しない ID（404 期待）..."
NOT_FOUND=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos/00000000-0000-0000-0000-000000000000")
if [ "$NOT_FOUND" = "404" ]; then
    pass "存在しない ID で 404"
else
    fail "期待: 404, 実際: $NOT_FOUND"
fi

# =============================================================================
# バッチ操作テスト
# =============================================================================
echo ""
echo "=== バッチ操作テスト ==="

echo ">>> POST /api/todos/batch - バッチ TODO 作成..."
BATCH_RESPONSE=$(curl -s -X POST "$EDGE_URL/api/todos/batch" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"todos": [{"title": "バッチ TODO 1"}, {"title": "バッチ TODO 2", "description": "説明"}, {"title": "バッチ TODO 3"}]}')
BATCH_COUNT=$(echo "$BATCH_RESPONSE" | grep -o '"id"' | wc -l | tr -d ' ')
if [ "$BATCH_COUNT" = "3" ]; then
    pass "バッチ TODO 作成: 3件"
    # 作成した TODO の ID を取得（後でクリーンアップ）
    BATCH_ID1=$(echo "$BATCH_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    BATCH_ID2=$(echo "$BATCH_RESPONSE" | grep -o '"id":"[^"]*"' | sed -n '2p' | cut -d'"' -f4)
    BATCH_ID3=$(echo "$BATCH_RESPONSE" | grep -o '"id":"[^"]*"' | tail -1 | cut -d'"' -f4)
else
    fail "バッチ TODO 作成失敗: $BATCH_RESPONSE"
fi

echo ">>> POST /api/todos/batch - 空配列（400 期待）..."
EMPTY_BATCH=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/todos/batch" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"todos": []}')
if [ "$EMPTY_BATCH" = "400" ]; then
    pass "空配列で 400"
else
    fail "期待: 400, 実際: $EMPTY_BATCH"
fi

echo ">>> POST /api/todos/batch - 無効なタイトル含む（400 期待）..."
INVALID_BATCH=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/todos/batch" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"todos": [{"title": "有効"}, {"title": ""}]}')
if [ "$INVALID_BATCH" = "400" ]; then
    pass "無効なタイトル含むバッチで 400"
else
    fail "期待: 400, 実際: $INVALID_BATCH"
fi

# =============================================================================
# ファイル付き TODO テスト
# =============================================================================
echo ""
echo "=== ファイル付き TODO テスト ==="

echo ">>> POST /api/todos/with-files - ファイル付き TODO 作成..."
FILES_RESPONSE=$(curl -s -X POST "$EDGE_URL/api/todos/with-files" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
        "title": "ファイル付き TODO",
        "description": "テスト",
        "files": [
            {"filename": "test.pdf", "mime_type": "application/pdf", "size_bytes": 12345, "storage_path": "/uploads/test.pdf"},
            {"filename": "image.png", "mime_type": "image/png", "size_bytes": 6789, "storage_path": "/uploads/image.png"}
        ]
    }')
if echo "$FILES_RESPONSE" | grep -q '"todo"' && echo "$FILES_RESPONSE" | grep -q '"files"'; then
    FILES_TODO_ID=$(echo "$FILES_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    pass "ファイル付き TODO 作成: $FILES_TODO_ID"
else
    fail "ファイル付き TODO 作成失敗: $FILES_RESPONSE"
fi

echo ">>> POST /api/todos/with-files - 無効なファイル名（400 期待）..."
INVALID_FILE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/todos/with-files" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
        "title": "無効なファイル",
        "files": [{"filename": "", "mime_type": "text/plain", "size_bytes": 100, "storage_path": "/test"}]
    }')
if [ "$INVALID_FILE" = "400" ]; then
    pass "無効なファイル名で 400"
else
    fail "期待: 400, 実際: $INVALID_FILE"
fi

echo ">>> POST /api/todos/with-files - パストラバーサル（400 期待）..."
PATH_TRAVERSAL=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$EDGE_URL/api/todos/with-files" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
        "title": "パストラバーサル",
        "files": [{"filename": "../../../etc/passwd", "mime_type": "text/plain", "size_bytes": 100, "storage_path": "/test"}]
    }')
if [ "$PATH_TRAVERSAL" = "400" ]; then
    pass "パストラバーサルで 400"
else
    fail "期待: 400, 実際: $PATH_TRAVERSAL"
fi

# =============================================================================
# クリーンアップ
# =============================================================================
echo ""
echo ">>> クリーンアップ中..."
for id in $BATCH_ID1 $BATCH_ID2 $BATCH_ID3 $FILES_TODO_ID; do
    if [ -n "$id" ]; then
        curl -s -o /dev/null -X DELETE -H "Authorization: Bearer $TOKEN" "$EDGE_URL/api/todos/$id"
    fi
done

# =============================================================================
# 結果サマリー
# =============================================================================
echo ""
echo "=============================================="
echo "テスト結果: ✅ $PASSED 成功 / ❌ $FAILED 失敗"
echo "=============================================="

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi
