# =============================================================================
# Makefile: spin-axum-todo 開発用コマンド
# =============================================================================
# 使用方法: make help
# =============================================================================

.PHONY: help setup up down migrate build build-core build-edge \
        run run-core run-edge test test-edge test-core test-all status demo clean expand logs \
        s3-ls s3-create-bucket

# デフォルトターゲット
.DEFAULT_GOAL := help

# 色付き出力
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
RESET := \033[0m

# 環境変数（.env から読み込み）
-include .env
export

# デフォルト値（.env がない場合のフォールバック）
APP_ADDR ?= 0.0.0.0:3001
DATABASE_WRITER_URL ?= postgres://app:app@localhost:5432/app
DATABASE_READER_URL ?= $(DATABASE_WRITER_URL)
REDIS_URL ?= redis://localhost:6379
JWT_SECRET ?= super-secret-key
JWT_EXPIRY_HOURS ?= 24
EDGE_SECRET ?= super-secret-edge-key
RUST_LOG ?= info

# S3/LocalStack 設定
S3_ENDPOINT_URL ?= http://localhost:4566
S3_BUCKET ?= todo-files
AWS_DEFAULT_REGION ?= ap-northeast-1
AWS_ACCESS_KEY_ID ?= test
AWS_SECRET_ACCESS_KEY ?= test

# =============================================================================
# ヘルプ
# =============================================================================

help: ## このヘルプを表示
	@echo ""
	@echo "$(CYAN)spin-axum-todo$(RESET) - 開発コマンド一覧"
	@echo ""
	@echo "$(GREEN)セットアップ:$(RESET)"
	@grep -E '^(setup|up|down|migrate):.*?## .*$$' Makefile | sed 's/:.*##/:##/' | awk 'BEGIN {FS = ":##"}; {printf "  $(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)ビルド:$(RESET)"
	@grep -E '^(build|build-core|build-edge|expand):.*?## .*$$' Makefile | sed 's/:.*##/:##/' | awk 'BEGIN {FS = ":##"}; {printf "  $(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)実行:$(RESET)"
	@grep -E '^(run|run-core|run-edge):.*?## .*$$' Makefile | sed 's/:.*##/:##/' | awk 'BEGIN {FS = ":##"}; {printf "  $(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)テスト:$(RESET)"
	@grep -E '^(test|test-edge|test-core|test-all):.*?## .*$$' Makefile | sed 's/:.*##/:##/' | awk 'BEGIN {FS = ":##"}; {printf "  $(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)ユーティリティ:$(RESET)"
	@grep -E '^(status|demo|logs|clean):.*?## .*$$' Makefile | sed 's/:.*##/:##/' | awk 'BEGIN {FS = ":##"}; {printf "  $(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)S3/LocalStack:$(RESET)"
	@grep -E '^(s3-ls|s3-create-bucket):.*?## .*$$' Makefile | sed 's/:.*##/:##/' | awk 'BEGIN {FS = ":##"}; {printf "  $(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""

# =============================================================================
# セットアップ
# =============================================================================

setup: .env up migrate build ## 初期セットアップ（.env作成、DB起動、マイグレーション、ビルド）
	@echo ""
	@echo "$(GREEN)セットアップ完了！$(RESET)"
	@echo "  make run-core  # Core 層を起動"
	@echo "  make run-edge  # Edge 層を起動（別ターミナル）"
	@echo ""
	@echo "$(YELLOW)認証フロー:$(RESET)"
	@echo "  1. ユーザー登録: POST /api/auth/register"
	@echo "  2. ログイン:     POST /api/auth/login -> JWT 取得"
	@echo "  3. API アクセス: Authorization: Bearer <JWT>"

.env: .env.example
	@echo ">>> .env ファイルを作成中..."
	cp .env.example .env
	@echo "    $(GREEN).env ファイルを作成しました$(RESET)"

up: ## インフラ起動（PostgreSQL + Redis + LocalStack）
	@echo ">>> Docker Compose でインフラを起動中..."
	docker compose up -d
	@echo ">>> PostgreSQL の起動を待機中..."
	@until docker compose exec -T postgres pg_isready -U app -d app > /dev/null 2>&1; do sleep 1; done
	@echo "    $(GREEN)PostgreSQL 起動完了$(RESET)"
	@echo ">>> Redis の起動を待機中..."
	@until docker compose exec -T redis redis-cli ping > /dev/null 2>&1; do sleep 1; done
	@echo "    $(GREEN)Redis 起動完了$(RESET)"
	@echo ">>> LocalStack (S3) の起動を待機中..."
	@until curl -s http://localhost:4566/_localstack/health | grep -q '"s3": *"running"' 2>/dev/null; do sleep 1; done
	@echo "    $(GREEN)LocalStack 起動完了$(RESET)"
	@$(MAKE) s3-create-bucket

down: ## インフラ停止
	docker compose down

migrate: ## マイグレーション実行
	@echo ">>> マイグレーションを実行中..."
	cd core/api && DATABASE_URL="$(DATABASE_WRITER_URL)" sqlx migrate run
	@echo "    $(GREEN)マイグレーション完了$(RESET)"

# =============================================================================
# ビルド
# =============================================================================

build: build-core build-edge ## 全てビルド（Core + Edge）

build-core: ## Core 層をビルド
	@echo ">>> Core 層をビルド中..."
	cd core && cargo build -p api
	@echo "    $(GREEN)Core 層ビルド完了$(RESET)"

build-edge: ## Edge 層をビルド
	@echo ">>> Edge 層をビルド中..."
	cd edge && spin build
	@echo "    $(GREEN)Edge 層ビルド完了$(RESET)"

expand: ## cargo expand で展開コードを生成
	@echo ">>> auth コンポーネントを展開中..."
	cd edge && cargo expand -p auth --target wasm32-wasip1 2>/dev/null > auth/expanded.rs
	@echo ">>> gateway コンポーネントを展開中..."
	cd edge && cargo expand -p gateway --target wasm32-wasip1 2>/dev/null > gateway/expanded.rs
	@echo "    $(GREEN)展開完了$(RESET)"
	@wc -l edge/auth/expanded.rs edge/gateway/expanded.rs

# =============================================================================
# 実行
# =============================================================================

run: ## Core と Edge を同時起動（フォアグラウンド）
	@echo "$(YELLOW)注意: 2つのターミナルで別々に起動することを推奨$(RESET)"
	@echo "  make run-core  # ターミナル1"
	@echo "  make run-edge  # ターミナル2"

run-core: ## Core 層を起動
	@echo ">>> Core 層を起動中..."
	@echo "    APP_ADDR=$(APP_ADDR)"
	@echo "    DATABASE_WRITER_URL=$(DATABASE_WRITER_URL)"
	@echo "    DATABASE_READER_URL=$(DATABASE_READER_URL)"
	@echo "    REDIS_URL=$(REDIS_URL)"
	@echo "    S3_ENDPOINT_URL=$(S3_ENDPOINT_URL)"
	@echo "    S3_BUCKET=$(S3_BUCKET)"
	@echo "    JWT_SECRET=****"
	@echo "    JWT_EXPIRY_HOURS=$(JWT_EXPIRY_HOURS)"
	@echo "    RUST_LOG=$(RUST_LOG)"
	cd core && \
		APP_ADDR=$(APP_ADDR) \
		DATABASE_WRITER_URL=$(DATABASE_WRITER_URL) \
		DATABASE_READER_URL=$(DATABASE_READER_URL) \
		REDIS_URL=$(REDIS_URL) \
		S3_ENDPOINT_URL=$(S3_ENDPOINT_URL) \
		S3_BUCKET=$(S3_BUCKET) \
		AWS_DEFAULT_REGION=$(AWS_DEFAULT_REGION) \
		AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
		AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
		JWT_SECRET=$(JWT_SECRET) \
		JWT_EXPIRY_HOURS=$(JWT_EXPIRY_HOURS) \
		EDGE_SECRET=$(EDGE_SECRET) \
		RUST_LOG=$(RUST_LOG) \
		cargo run -p api

run-edge: ## Edge 層を起動
	@echo ">>> Edge 層をビルド・起動中..."
	cd edge && spin build && spin up

# =============================================================================
# テスト・ユーティリティ
# =============================================================================

test: test-edge ## 統合テストを実行（Edge 層経由）

test-edge: ## Edge 層経由で全エンドポイントをテスト
	@echo ">>> Edge 層統合テストを実行中..."
	./scripts/test-edge.sh

test-core: ## Core 層単体で全エンドポイントをテスト
	@echo ">>> Core 層単体テストを実行中..."
	./scripts/test-core.sh

test-all: test-core test-edge ## Core + Edge 両方のテストを実行

status: ## サービスの稼働状況を確認
	@echo "=== サービス稼働状況 ==="
	@echo ""
	@echo "$(CYAN)Docker Compose:$(RESET)"
	@docker compose ps --format "table {{.Name}}\t{{.Status}}\t{{.Ports}}" 2>/dev/null || echo "  $(RED)Docker Compose 未起動$(RESET)"
	@echo ""
	@echo "$(CYAN)LocalStack (S3):$(RESET)"
	@curl -s http://localhost:4566/_localstack/health 2>/dev/null | grep -o '"s3": *"[^"]*"' || echo "  $(RED)未起動$(RESET)"
	@echo ""
	@echo "$(CYAN)Core 層 (port 3001):$(RESET)"
	@curl -s http://localhost:3001/health 2>/dev/null && echo "" || echo "  $(RED)未起動$(RESET)"
	@echo ""
	@echo "$(CYAN)Edge 層 (port 3000):$(RESET)"
	@curl -s http://localhost:3000/health 2>/dev/null && echo "" || echo "  $(RED)未起動$(RESET)"

demo: ## 認証フローのデモ（ユーザー登録 → ログイン → TODO 操作）
	@echo "=== 認証フローデモ ==="
	@echo ""
	@EMAIL="demo-$$(date +%s)@example.com"; \
	PASSWORD="password123"; \
	echo "$(CYAN)1. ユーザー登録$(RESET)"; \
	echo "   POST /api/auth/register"; \
	echo "   Email: $$EMAIL"; \
	REGISTER=$$(curl -s -X POST http://localhost:3000/api/auth/register \
		-H "Content-Type: application/json" \
		-d "{\"email\": \"$$EMAIL\", \"password\": \"$$PASSWORD\"}"); \
	echo "   Response: $$REGISTER"; \
	echo ""; \
	echo "$(CYAN)2. ログイン$(RESET)"; \
	echo "   POST /api/auth/login"; \
	TOKEN=$$(curl -s -X POST http://localhost:3000/api/auth/login \
		-H "Content-Type: application/json" \
		-d "{\"email\": \"$$EMAIL\", \"password\": \"$$PASSWORD\"}" | grep -o '"token":"[^"]*"' | cut -d'"' -f4); \
	echo "   Token: $${TOKEN:0:50}..."; \
	echo ""; \
	echo "$(CYAN)3. TODO 作成$(RESET)"; \
	echo "   POST /api/todos"; \
	TODO=$$(curl -s -X POST http://localhost:3000/api/todos \
		-H "Authorization: Bearer $$TOKEN" \
		-H "Content-Type: application/json" \
		-d '{"title": "デモ TODO", "description": "make demo で作成"}'); \
	echo "   Response: $$TODO"; \
	TODO_ID=$$(echo "$$TODO" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4); \
	echo ""; \
	echo "$(CYAN)4. TODO 一覧取得$(RESET)"; \
	echo "   GET /api/todos"; \
	curl -s -H "Authorization: Bearer $$TOKEN" http://localhost:3000/api/todos; \
	echo ""; \
	echo ""; \
	echo "$(GREEN)デモ完了！$(RESET)"; \
	echo "JWT トークン（1時間有効）:"; \
	echo "$$TOKEN"

logs: ## Spin ログを表示
	@if [ -d "edge/.spin/logs" ]; then \
		tail -f edge/.spin/logs/*.txt; \
	else \
		echo "ログファイルがありません。Edge 層を起動してください。"; \
	fi

clean: ## ビルド成果物を削除
	@echo ">>> ビルド成果物を削除中..."
	cd core && cargo clean
	rm -rf edge/target edge/.spin
	@echo "    $(GREEN)クリーン完了$(RESET)"

# =============================================================================
# S3/LocalStack
# =============================================================================

# AWS CLI を Docker 経由で実行（ローカルインストール不要）
AWS_CLI = docker compose exec -T localstack awslocal

s3-ls: ## S3 バケット内のファイル一覧を表示
	@echo ">>> S3 バケット一覧..."
	@$(AWS_CLI) s3 ls 2>/dev/null || echo "  $(RED)LocalStack 未起動$(RESET)"
	@echo ""
	@echo ">>> $(S3_BUCKET) バケット内のファイル..."
	@$(AWS_CLI) s3 ls s3://$(S3_BUCKET)/ --recursive 2>/dev/null || echo "  $(YELLOW)バケットが存在しないか空です$(RESET)"

s3-create-bucket: ## S3 バケットを作成（LocalStack 用）
	@echo ">>> S3 バケット '$(S3_BUCKET)' を確認中..."
	@if $(AWS_CLI) s3 ls s3://$(S3_BUCKET) > /dev/null 2>&1; then \
		echo "    $(GREEN)バケット '$(S3_BUCKET)' は既に存在します$(RESET)"; \
	else \
		echo ">>> バケット '$(S3_BUCKET)' を作成中..."; \
		$(AWS_CLI) s3 mb s3://$(S3_BUCKET) && \
		echo "    $(GREEN)バケット '$(S3_BUCKET)' を作成しました$(RESET)"; \
	fi
