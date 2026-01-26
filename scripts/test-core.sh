#!/bin/bash
# =============================================================================
# test-core.sh: Core 層単体テスト
# =============================================================================
# Edge 層を介さず、Core 層に直接アクセスして全 API エンドポイントをテストする。
# X-User-Id と X-Edge-Verified ヘッダーを手動で設定してリクエストする。
#
# 前提条件:
#   - Core 層が localhost:3001 で起動していること
#   - Edge 層は不要
#
# 使用方法:
#   ./scripts/test-core.sh
# =============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

CORE_URL="http://localhost:3001"

# .env から EDGE_SECRET を読み込み
if [ -f "$PROJECT_ROOT/.env" ]; then
    export $(cat "$PROJECT_ROOT/.env" | grep -v '^#' | grep EDGE_SECRET | xargs)
fi
EDGE_SECRET="${EDGE_SECRET:-super-secret-edge-key}"

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
TEST_EMAIL="core-test-$(date +%s)@example.com"
TEST_PASSWORD="password123"

echo "=== spin-axum-todo Core 層単体テスト ==="
echo "    テストユーザー: $TEST_EMAIL"
echo "    Core URL: $CORE_URL"

# =============================================================================
# ヘルスチェック
# =============================================================================
echo ""
echo "=== ヘルスチェック ==="

echo ">>> GET /health..."
HEALTH=$(curl -s "$CORE_URL/health" || echo "FAILED")
if echo "$HEALTH" | grep -q "ok"; then
    pass "Health check"
else
    fail "Health check: $HEALTH"
    echo "    Core 層が起動していない可能性があります"
    exit 1
fi

# =============================================================================
# 認証 API テスト（Edge 検証不要）
# =============================================================================
echo ""
echo "=== 認証 API テスト ==="

echo ">>> POST /api/auth/register - ユーザー登録..."
REGISTER_RESPONSE=$(curl -s -X POST "$CORE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\", \"display_name\": \"Core Test User\"}")
if echo "$REGISTER_RESPONSE" | grep -q '"id"'; then
    USER_ID=$(echo "$REGISTER_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
    pass "ユーザー登録成功: $USER_ID"
else
    fail "ユーザー登録失敗: $REGISTER_RESPONSE"
    exit 1
fi

echo ">>> POST /api/auth/register - 重複メールアドレス（409 期待）..."
DUPLICATE_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$CORE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}")
if [ "$DUPLICATE_RESPONSE" = "409" ]; then
    pass "重複メールアドレスで 409"
else
    fail "期待: 409, 実際: $DUPLICATE_RESPONSE"
fi

echo ">>> POST /api/auth/register - 無効なメール形式（400 期待）..."
INVALID_EMAIL=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$CORE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d '{"email": "invalid-email", "password": "password123"}')
if [ "$INVALID_EMAIL" = "400" ]; then
    pass "無効なメール形式で 400"
else
    fail "期待: 400, 実際: $INVALID_EMAIL"
fi

echo ">>> POST /api/auth/register - 短いパスワード（400 期待）..."
SHORT_PASSWORD=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$CORE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d '{"email": "short@example.com", "password": "short"}')
if [ "$SHORT_PASSWORD" = "400" ]; then
    pass "短いパスワードで 400"
else
    fail "期待: 400, 実際: $SHORT_PASSWORD"
fi

echo ">>> POST /api/auth/login - ログイン..."
LOGIN_RESPONSE=$(curl -s -X POST "$CORE_URL/api/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"$TEST_PASSWORD\"}")
if echo "$LOGIN_RESPONSE" | grep -q '"token"'; then
    pass "ログイン成功"
else
    fail "ログイン失敗: $LOGIN_RESPONSE"
    exit 1
fi

echo ">>> POST /api/auth/login - 誤ったパスワード（401 期待）..."
WRONG_PASSWORD=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$CORE_URL/api/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$TEST_EMAIL\", \"password\": \"wrongpassword\"}")
if [ "$WRONG_PASSWORD" = "401" ]; then
    pass "誤ったパスワードで 401"
else
    fail "期待: 401, 実際: $WRONG_PASSWORD"
fi

# =============================================================================
# Edge 検証テスト
# =============================================================================
echo ""
echo "=== Edge 検証テスト ==="

echo ">>> GET /api/todos - X-Edge-Verified なし（403 期待）..."
NO_EDGE=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "X-User-Id: $USER_ID" \
    "$CORE_URL/api/todos")
if [ "$NO_EDGE" = "403" ]; then
    pass "X-Edge-Verified なしで 403"
else
    fail "期待: 403, 実際: $NO_EDGE"
fi

echo ">>> GET /api/todos - 無効な X-Edge-Verified（403 期待）..."
INVALID_EDGE=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: wrong-secret" \
    "$CORE_URL/api/todos")
if [ "$INVALID_EDGE" = "403" ]; then
    pass "無効な X-Edge-Verified で 403"
else
    fail "期待: 403, 実際: $INVALID_EDGE"
fi

echo ">>> GET /api/todos - X-User-Id なし（401 期待）..."
NO_USER_ID=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos")
if [ "$NO_USER_ID" = "401" ]; then
    pass "X-User-Id なしで 401"
else
    fail "期待: 401, 実際: $NO_USER_ID"
fi

echo ">>> GET /api/todos - 無効な X-User-Id 形式（401 期待）..."
INVALID_USER_ID=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "X-User-Id: not-a-uuid" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos")
if [ "$INVALID_USER_ID" = "401" ]; then
    pass "無効な X-User-Id 形式で 401"
else
    fail "期待: 401, 実際: $INVALID_USER_ID"
fi

# =============================================================================
# TODO CRUD テスト（Core 層直接アクセス）
# =============================================================================
echo ""
echo "=== TODO CRUD テスト ==="

# 共通ヘッダー
AUTH_HEADERS="-H \"X-User-Id: $USER_ID\" -H \"X-Edge-Verified: $EDGE_SECRET\""

echo ">>> GET /api/todos - 一覧取得..."
TODOS=$(curl -s \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos")
if echo "$TODOS" | grep -q "\["; then
    pass "TODO 一覧を取得"
else
    fail "TODO 一覧取得失敗: $TODOS"
fi

echo ">>> POST /api/todos - TODO 作成..."
CREATE_RESPONSE=$(curl -s -X POST \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{"title": "Core テスト TODO", "description": "Core 層単体テストで作成"}' \
    "$CORE_URL/api/todos")
TODO_ID=$(echo "$CREATE_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
if [ -n "$TODO_ID" ]; then
    pass "TODO 作成: $TODO_ID"
else
    fail "TODO 作成失敗: $CREATE_RESPONSE"
    exit 1
fi

echo ">>> POST /api/todos - 空タイトル（400 期待）..."
EMPTY_TITLE=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{"title": ""}' \
    "$CORE_URL/api/todos")
if [ "$EMPTY_TITLE" = "400" ]; then
    pass "空タイトルで 400"
else
    fail "期待: 400, 実際: $EMPTY_TITLE"
fi

echo ">>> GET /api/todos/{id} - TODO 取得..."
GET_RESPONSE=$(curl -s \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos/$TODO_ID")
if echo "$GET_RESPONSE" | grep -q "$TODO_ID"; then
    pass "TODO 取得"
else
    fail "TODO 取得失敗: $GET_RESPONSE"
fi

echo ">>> GET /api/todos?completed=false - 未完了フィルタ..."
INCOMPLETE=$(curl -s \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos?completed=false")
if echo "$INCOMPLETE" | grep -q "$TODO_ID"; then
    pass "未完了フィルタで TODO を取得"
else
    fail "未完了フィルタで TODO が見つからない"
fi

echo ">>> PATCH /api/todos/{id} - タイトル更新..."
PATCH_TITLE=$(curl -s -X PATCH \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{"title": "Core 更新タイトル"}' \
    "$CORE_URL/api/todos/$TODO_ID")
if echo "$PATCH_TITLE" | grep -q 'Core 更新タイトル'; then
    pass "タイトル更新"
else
    fail "タイトル更新失敗: $PATCH_TITLE"
fi

echo ">>> PATCH /api/todos/{id} - 完了に更新..."
PATCH_COMPLETE=$(curl -s -X PATCH \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{"completed": true}' \
    "$CORE_URL/api/todos/$TODO_ID")
if echo "$PATCH_COMPLETE" | grep -q '"completed":true'; then
    pass "完了に更新"
else
    fail "完了に更新失敗: $PATCH_COMPLETE"
fi

echo ">>> GET /api/todos?completed=true - 完了フィルタ..."
COMPLETE=$(curl -s \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos?completed=true")
if echo "$COMPLETE" | grep -q "$TODO_ID"; then
    pass "完了フィルタで TODO を取得"
else
    fail "完了フィルタで TODO が見つからない"
fi

echo ">>> DELETE /api/todos/{id} - TODO 削除..."
DELETE_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos/$TODO_ID")
if [ "$DELETE_RESPONSE" = "204" ]; then
    pass "TODO 削除"
else
    fail "TODO 削除失敗: HTTP $DELETE_RESPONSE"
fi

echo ">>> GET /api/todos/{id} - 削除後取得（404 期待）..."
DELETED=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos/$TODO_ID")
if [ "$DELETED" = "404" ]; then
    pass "削除後 404"
else
    fail "期待: 404, 実際: $DELETED"
fi

# =============================================================================
# バッチ操作テスト
# =============================================================================
echo ""
echo "=== バッチ操作テスト ==="

echo ">>> POST /api/todos/batch - バッチ TODO 作成..."
BATCH_RESPONSE=$(curl -s -X POST \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{"todos": [{"title": "Core バッチ 1"}, {"title": "Core バッチ 2"}, {"title": "Core バッチ 3"}]}' \
    "$CORE_URL/api/todos/batch")
BATCH_COUNT=$(echo "$BATCH_RESPONSE" | grep -o '"id"' | wc -l | tr -d ' ')
if [ "$BATCH_COUNT" = "3" ]; then
    pass "バッチ TODO 作成: 3件"
    BATCH_ID1=$(echo "$BATCH_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    BATCH_ID2=$(echo "$BATCH_RESPONSE" | grep -o '"id":"[^"]*"' | sed -n '2p' | cut -d'"' -f4)
    BATCH_ID3=$(echo "$BATCH_RESPONSE" | grep -o '"id":"[^"]*"' | tail -1 | cut -d'"' -f4)
else
    fail "バッチ TODO 作成失敗: $BATCH_RESPONSE"
fi

echo ">>> POST /api/todos/batch - 空配列（400 期待）..."
EMPTY_BATCH=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{"todos": []}' \
    "$CORE_URL/api/todos/batch")
if [ "$EMPTY_BATCH" = "400" ]; then
    pass "空配列で 400"
else
    fail "期待: 400, 実際: $EMPTY_BATCH"
fi

# =============================================================================
# ファイル付き TODO テスト
# =============================================================================
echo ""
echo "=== ファイル付き TODO テスト ==="

echo ">>> POST /api/todos/with-files - ファイル付き TODO 作成..."
FILES_RESPONSE=$(curl -s -X POST \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{
        "title": "Core ファイル付き TODO",
        "description": "テスト",
        "files": [
            {"filename": "core-test.pdf", "mime_type": "application/pdf", "size_bytes": 12345, "storage_path": "/uploads/core-test.pdf"}
        ]
    }' \
    "$CORE_URL/api/todos/with-files")
if echo "$FILES_RESPONSE" | grep -q '"todo"' && echo "$FILES_RESPONSE" | grep -q '"files"'; then
    FILES_TODO_ID=$(echo "$FILES_RESPONSE" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    pass "ファイル付き TODO 作成: $FILES_TODO_ID"
else
    fail "ファイル付き TODO 作成失敗: $FILES_RESPONSE"
fi

echo ">>> POST /api/todos/with-files - 無効な MIME タイプ（400 期待）..."
INVALID_MIME=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{
        "title": "無効な MIME",
        "files": [{"filename": "test.txt", "mime_type": "invalid", "size_bytes": 100, "storage_path": "/test"}]
    }' \
    "$CORE_URL/api/todos/with-files")
if [ "$INVALID_MIME" = "400" ]; then
    pass "無効な MIME タイプで 400"
else
    fail "期待: 400, 実際: $INVALID_MIME"
fi

# =============================================================================
# 所有権テスト（別ユーザーの TODO にアクセス）
# =============================================================================
echo ""
echo "=== 所有権テスト ==="

# 別のユーザーを作成
OTHER_EMAIL="other-$(date +%s)@example.com"
curl -s -X POST "$CORE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$OTHER_EMAIL\", \"password\": \"$TEST_PASSWORD\"}" > /dev/null
OTHER_USER_ID=$(curl -s -X POST "$CORE_URL/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"other2-$(date +%s)@example.com\", \"password\": \"$TEST_PASSWORD\"}" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

# 元のユーザーで TODO を作成
OWNER_TODO=$(curl -s -X POST \
    -H "X-User-Id: $USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    -H "Content-Type: application/json" \
    -d '{"title": "所有権テスト TODO"}' \
    "$CORE_URL/api/todos")
OWNER_TODO_ID=$(echo "$OWNER_TODO" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)

echo ">>> GET /api/todos/{id} - 他ユーザーの TODO（404 期待）..."
OTHER_ACCESS=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "X-User-Id: $OTHER_USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos/$OWNER_TODO_ID")
if [ "$OTHER_ACCESS" = "404" ]; then
    pass "他ユーザーの TODO で 404"
else
    fail "期待: 404, 実際: $OTHER_ACCESS"
fi

echo ">>> DELETE /api/todos/{id} - 他ユーザーの TODO（404 期待）..."
OTHER_DELETE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE \
    -H "X-User-Id: $OTHER_USER_ID" \
    -H "X-Edge-Verified: $EDGE_SECRET" \
    "$CORE_URL/api/todos/$OWNER_TODO_ID")
if [ "$OTHER_DELETE" = "404" ]; then
    pass "他ユーザーの TODO 削除で 404"
else
    fail "期待: 404, 実際: $OTHER_DELETE"
fi

# =============================================================================
# クリーンアップ
# =============================================================================
echo ""
echo ">>> クリーンアップ中..."
for id in $BATCH_ID1 $BATCH_ID2 $BATCH_ID3 $FILES_TODO_ID $OWNER_TODO_ID; do
    if [ -n "$id" ]; then
        curl -s -o /dev/null -X DELETE \
            -H "X-User-Id: $USER_ID" \
            -H "X-Edge-Verified: $EDGE_SECRET" \
            "$CORE_URL/api/todos/$id"
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
