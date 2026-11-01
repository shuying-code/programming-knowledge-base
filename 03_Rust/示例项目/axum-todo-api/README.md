# axum-todo-api（示例项目）

基于 **Axum 0.8 + Tokio** 的最小 Todo REST API，使用内存存储（`Arc<RwLock<HashMap>>`），**无需数据库**即可直接运行，用于演示常见的 Axum 工程用法。

## 运行

```bash
cargo run
# 服务默认监听 0.0.0.0:3000
```

## 接口

| 方法 | 路径 | 说明 |
|---|---|---|
| GET | `/health` | 健康检查 |
| GET | `/todos` | 列表 |
| POST | `/todos` | 新建（body: `{"title":"..."}`） |
| GET | `/todos/{id}` | 详情 |
| PUT | `/todos/{id}` | 更新（body: `{"title":"?","done":true}` 可选字段） |
| DELETE | `/todos/{id}` | 删除 |

> 注意：Axum 0.8 的路径参数是 `/{id}`，旧版 `/:id` 写法会 panic。

## 体验

```bash
curl -X POST localhost:3000/todos -H 'content-type: application/json' -d '{"title":"learn rust"}'
curl localhost:3000/todos
curl localhost:3000/todos/<id>
curl -X PUT localhost:3000/todos/<id> -H 'content-type: application/json' -d '{"done":true}'
curl -X DELETE localhost:3000/todos/<id>
```

## 覆盖要点

- 路由分组与方法链：`get(...).post(...)`、`get(...).put(...).delete(...)`
- 提取器：`Path<Uuid>`、`Json<T>`、`State<AppState>`（body 提取器在最后）
- 统一错误：`AppError` 实现 `IntoResponse`
- 中间件：`TraceLayer`（请求日志）、`CorsLayer`
- 优雅停机：`with_graceful_shutdown` 监听 Ctrl+C

## 下一步

把内存存储替换为 `sqlx` + 数据库、加入分页/鉴权/参数校验，即可演进为真实服务。
