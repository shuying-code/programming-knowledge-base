# Rust 常用框架与实战指南

> 面向后端开发的 Rust 生态实战整理：异步运行时、Web 框架、数据层、序列化、日志、中间件与工程结构。
> 版本以 2026-11 的 crates.io 稳定版为准：`tokio 1.x`、`axum 0.8.x`、`actix-web 4.x`、`sqlx 0.9.x`、`serde 1.x`、`tower-http 0.7.x`、`tracing 0.1.x`。
> **【注意】axum 0.8 起路径参数语法由 `/:id` 改为 `/{id}`，通配符由 `/*rest` 改为 `/{*rest}`，旧写法会 panic。**
> 配套源码见本目录 `示例项目/axum-todo-api/`。

## 目录

1. 生态与选型
2. Tokio 运行时基础
3. Axum Web 框架（0.8）
4. Actix-web 4 简述
5. 数据层 sqlx
6. 序列化 serde
7. 日志与可观测性 tracing
8. 中间件 tower / tower-http
9. 配置、错误与分层结构
10. 部署与运维
11. 示例项目说明

---

## 一、生态与选型

| 领域 | 常用库 | 说明 |
|---|---|---|
| 异步运行时 | **Tokio**（主流）、async-std、smol | Tokio 生态最完整 |
| Web 框架 | **Axum**（tower 生态、类型安全）、**Actix-web**（性能强）、Rocket、Warp | Axum 与 Tokio 同源，推荐新项目 |
| 数据库 | **sqlx**（异步、编译期校验 SQL）、SeaORM、Diesel | sqlx 无需 ORM、贴近 SQL |
| 序列化 | **serde** / serde_json | 事实标准 |
| 日志/追踪 | **tracing** + tracing-subscriber | 结构化、异步友好 |
| 中间件 | tower / tower-http | Axum 原生兼容 |
| 配置 | figment / config / envy | 多来源配置合并 |
| 错误 | thiserror（库）/ anyhow（应用） | |
| CLI | clap | |

**选型建议**：新项目首选 **Axum + Tokio + sqlx + serde + tracing**；追求极致吞吐或已有 Actor 风格偏好可用 Actix-web。

---

## 二、Tokio 运行时基础

```rust
#[tokio::main]           // 默认多线程运行时
async fn main() {
    let h = tokio::spawn(async { 1 + 1 });   // 派生任务
    let v = h.await.unwrap();
    println!("{v}");
}
```

- `#[tokio::main]` 是宏，等价于构建 `Runtime` 并 `block_on`。CPU 单线程可用 `#[tokio::main(flavor = "current_thread")]`。
- `tokio::spawn` 要求 Future `Send + 'static`（多线程运行时）。
- 组合并发：`tokio::join!`（全部完成）、`tokio::try_join!`（任一失败即返回）、`tokio::select!`（谁先完成用谁）。
- 超时：`tokio::time::timeout(dur, fut).await`。
- **阻塞操作**放 `tokio::task::spawn_blocking(|| ...)`，切勿在 async 中同步阻塞。

```rust
use tokio::time::{sleep, Duration, timeout};
let r = timeout(Duration::from_secs(1), sleep(Duration::from_secs(5))).await; // Err(Elapsed)
```

---

## 三、Axum Web 框架（0.8）

Axum 基于 **tower**（中间件）与 **hyper**（HTTP），核心是「路由 + 处理函数 + 提取器（Extractor）+ 状态（State）」。

### 3.1 最小示例

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### 3.2 路由与路径参数（0.8 语法）

```rust
use axum::{routing::get, Router, extract::Path};

async fn user(Path(id): Path<u64>) -> String { format!("user {id}") }
async fn files(Path(rest): Path<String>) -> String { format!("file: {rest}") }

let app = Router::new()
    .route("/users/{id}", get(user))        // 单段参数（旧写法 /:id 已废弃）
    .route("/files/{*rest}", get(files))    // 通配符（旧写法 /*rest 已废弃）
    .route("/", get(|| async { "home" }));
```

### 3.3 提取器（Extractor）

常用提取器：`Path<T>`、`Query<T>`、`Json<T>`、`State<S>`、`HeaderMap`、`TypedHeader`。

- 提取器实现 `FromRequestParts`（不消耗 body）或 `FromRequest`（消耗 body）。
- **【注意】** 消耗 body 的提取器（如 `Json<T>`）必须是函数的**最后一个**参数。

```rust
use axum::{extract::{Query, State}, Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Page { page: Option<u32>, size: Option<u32> }

async fn list(State(state): State<AppState>, Query(p): Query<Page>) -> Json<Vec<Item>> {
    // ...
}
```

### 3.4 共享状态 State

```rust
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

type AppState = Arc<RwLock<HashMap<u64, String>>>;

let state: AppState = Arc::new(RwLock::new(HashMap::new()));
let app = Router::new()
    .route("/items/{id}", get(get_item))
    .with_state(state);
```

`with_state` 让所有 handler 通过 `State<S>` 取到；状态类型需 `Clone`（`Arc` 天然满足）。

### 3.5 错误处理（IntoResponse）

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

pub enum AppError { NotFound }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found"),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

// handler 返回 Result<Json<T>, AppError>，用 ? 自动转成响应
```

### 3.6 中间件（tower 层）

```rust
use tower_http::{trace::TraceLayer, cors::CorsLayer};

let app = Router::new()
    .route("/", get(|| async { "ok" }))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive());
```

自定义中间件可用 `axum::middleware::from_fn`。

### 3.7 优雅停机

```rust
axum::serve(listener, app)
    .with_graceful_shutdown(async {
        tokio::signal::ctrl_c().await.ok();
    })
    .await
    .unwrap();
```

---

## 四、Actix-web 4 简述

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
```

- Actix-web 基于 Actor 模型（`actix`），性能强；`App` 注册路由、中间件、`Data`（共享状态）。
- 与 Axum 差异：路由宏（`#[get]`）、`App`/`HttpServer` 结构、`web::Data<T>` 共享状态。
- 两者都用 Tokio（Actix 4 基于 Tokio）。

---

## 五、数据层 sqlx

sqlx 是异步、编译期校验 SQL 的库（支持 PostgreSQL / MySQL / SQLite）。

```rust
use sqlx::{sqlite::SqlitePoolOptions, FromRow};

#[derive(FromRow)]
struct User { id: i64, name: String }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://app.db").await?;

    let users: Vec<User> = sqlx::query_as("SELECT id, name FROM users")
        .fetch_all(&pool).await?;

    sqlx::query("INSERT INTO users(name) VALUES (?)")
        .bind("alice").execute(&pool).await?;
    Ok(())
}
```

- **编译期校验**：`sqlx::query!("SELECT ...")` 宏在编译时连库校验 SQL 与类型；无库环境用「离线模式」`cargo sqlx prepare` 生成 `.sqlx/`。
- **迁移**：`sqlx::migrate!("./migrations").run(&pool)`，迁移文件放 `migrations/`。
- 事务：`pool.begin().await?` 得到 `Transaction`，提交 `tx.commit()`，回滚可 `drop` 或 `rollback()`。

---

## 六、序列化 serde

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    id: u64,
    user_name: String,             // 序列化为 userName
    #[serde(default)]
    active: bool,                  // 缺省为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}
```

常用属性：`rename` / `rename_all`、`default`、`skip`、`skip_serializing_if`、`flatten`、`tag`（枚举标记方式）。`serde_json` 负责 JSON，其它格式有 `toml`、`serde_yaml` 等。

---

## 七、日志与可观测性 tracing

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=debug".into()))
    .with(tracing_subscriber::fmt::layer())
    .init();

tracing::info!(user_id = 42, "user logged in");   // 结构化字段
```

- `tracing` 提供结构化、异步上下文友好的日志/span；`tracing-subscriber` 输出与过滤。
- 可接入 OpenTelemetry 导出到 Jaeger/Prometheus。

---

## 八、中间件 tower / tower-http

`tower` 是通用中间件抽象（`Service`），`tower-http` 提供开箱即用层：

`TraceLayer`、`CorsLayer`、`CompressionLayer`、`TimeoutLayer`、`LimitLayer`、`ServeDir`（静态文件）。

```rust
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, serve::ServeDir};
use std::time::Duration;

let app = Router::new()
    .route("/", get(|| async { "ok" }))
    .nest_service("/static", ServeDir::new("public"))
    .layer(CompressionLayer::new())
    .layer(TimeoutLayer::new(Duration::from_secs(10)));
```

---

## 九、配置、错误与分层结构

推荐分层（以 Axum 为例）：

```text
src/
├─ main.rs        启动：日志、配置、连接池、路由、serve
├─ config.rs      读取环境变量 / 配置文件
├─ state.rs       全局共享状态（AppState）
├─ error.rs       统一错误类型 + IntoResponse
├─ routes/         路由分组（mod + handler）
├─ handlers/       业务 handler
├─ models/         领域模型与 DTO（serde）
└─ services/       业务逻辑 / 仓储访问（sqlx）
```

- 配置读取：`std::env::var("DATABASE_URL")`，或用 `figment`/`config` 合并「默认值 + 文件 + 环境变量」。
- 错误：库/内部用 `thiserror` 定义枚举，`impl IntoResponse` 统一转 HTTP；应用入口用 `anyhow::Result`。
- **【注意】** 不要把 DB 连接、密钥写死在代码；通过环境变量注入。

---

## 十、部署与运维

- 构建：`cargo build --release`，产物在 `target/release/<bin>`。
- 静态链接（便于容器/跨发行版）：`rustup target add x86_64-unknown-linux-musl` + `cargo build --release --target x86_64-unknown-linux-musl`。
- Docker 多阶段构建：builder 阶段编译，runner 阶段只拷二进制（可用 `scratch`/`debian-slim`）。
- 健康检查：暴露 `/health`；优雅停机接收 `SIGTERM`/`Ctrl-C`。
- 交叉编译：`cross` 工具（基于 Docker）简化跨平台构建。

```dockerfile
# 多阶段构建示例
FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/axum-todo-api /usr/local/bin/app
EXPOSE 3000
CMD ["app"]
```

---

## 十一、示例项目说明

`示例项目/axum-todo-api/` 是一个可直接 `cargo run` 的最小 REST API：

- 技术栈：Axum 0.8 + Tokio + serde + tower-http + tracing + uuid，内存存储（`Arc<RwLock<HashMap>>`），无需数据库即可运行。
- 接口：`GET /health`、`GET /todos`、`POST /todos`、`GET /todos/{id}`、`PUT /todos/{id}`、`DELETE /todos/{id}`。
- 覆盖点：路由分组、`Path`/`Json`/`State` 提取器、统一错误 `IntoResponse`、`TraceLayer`/`CorsLayer`、优雅停机。

启动与体验：

```bash
cd 示例项目/axum-todo-api
cargo run
# 另开终端
curl -X POST localhost:3000/todos -H 'content-type: application/json' -d '{"title":"learn rust"}'
curl localhost:3000/todos
```

> 把内存存储替换为 sqlx + 数据库、增加鉴权与分页，即可演进为真实服务。

---

_整理自 Tokio / Axum / Actix-web / sqlx / tower / serde / tracing 官方文档，仅供学习参考。_
