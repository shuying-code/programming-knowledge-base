# Go 常用框架与实战指南

> 面向后端开发的 Go 生态实战整理：标准库 HTTP、Web 框架（Gin/Echo）、ORM（GORM）、微服务（go-zero）、配置、日志、测试与性能分析、项目结构与部署。
> 说明：具体库版本以 `go get` 拉到的最新稳定版为准（Gin 为 v1.x、GORM 为 v1.x、go-zero 为 v1.x）；本文聚焦用法与工程模式。
> 配套文档：`Go知识笔记合集（整理版）.md`、`Go面试题（精编版）.md`。

## 目录

1. 生态与选型
2. 标准库 net/http 与优雅停机
3. Gin Web 框架实战
4. Echo 简述
5. GORM 数据层
6. go-zero 微服务
7. 配置、日志与依赖注入
8. 测试与性能分析
9. 并发实战模式
10. 项目结构与部署

---

## 一、生态与选型

| 领域 | 常用库 | 说明 |
|---|---|---|
| Web 框架 | **Gin**（最常见）、Echo、Fiber、chi | Gin 生态成熟、上手快；Fiber 基于 fasthttp 性能高 |
| ORM / SQL | **GORM**、sqlx、ent | GORM 全功能 ORM；sqlx 贴近 SQL；ent 图式建模 |
| 微服务 | **go-zero**、Kratos、Kitex | go-zero 提供 api/rpc + 代码生成 |
| 配置 | **viper**、koanf、env | 多来源配置合并 |
| 日志 | 标准库 **log/slog**（1.21+）、zap、zerolog | slog 标准、结构化；zap 高性能 |
| DI | **google/wire**、uber/dig | wire 编译期生成、无反射 |
| 测试 | 标准 `testing`、**testify**、httptest | 表驱动 + 断言库 |
| 工具库 | `golang.org/x/sync`（errgroup/semaphore）、ants（协程池） | |

**选型建议**：单体/中小项目 **Gin + GORM + slog + viper**；微服务用 **go-zero**（自带 api/rpc、限流、熔断、链路追踪）。

---

## 二、标准库 net/http 与优雅停机

Go 1.22 起 `net/http` 的 `ServeMux` 支持「方法 + 通配路径」：

```go
package main

import (
    "context"
    "errors"
    "log/slog"
    "net/http"
    "os/signal"
    "syscall"
    "time"
)

func main() {
    mux := http.NewServeMux()
    mux.HandleFunc("GET /health", func(w http.ResponseWriter, r *http.Request) {
        w.Write([]byte("ok"))
    })
    mux.HandleFunc("GET /users/{id}", func(w http.ResponseWriter, r *http.Request) {
        id := r.PathValue("id") // Go 1.22+ 路径参数
        w.Write([]byte("user " + id))
    })

    srv := &http.Server{Addr: ":8080", Handler: mux}

    ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
    defer stop()

    go func() {
        if err := srv.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
            slog.Error("server error", "err", err)
        }
    }()

    <-ctx.Done() // 等待退出信号
    shutdownCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
    defer cancel()
    _ = srv.Shutdown(shutdownCtx) // 优雅停机
}
```

---

## 三、Gin Web 框架实战

```go
package main

import (
    "net/http"
    "github.com/gin-gonic/gin"
)

type CreateUser struct {
    Name string `json:"name" binding:"required"`
}

func main() {
    r := gin.New()
    r.Use(gin.Logger(), gin.Recovery()) // 日志 + panic 恢复中间件

    api := r.Group("/api")
    {
        api.GET("/users/:id", getUser)      // Gin 路径参数用 :id
        api.POST("/users", createUser)
    }
    r.Run(":8080")
}

func getUser(c *gin.Context) {
    id := c.Param("id")
    c.JSON(http.StatusOK, gin.H{"id": id})
}

func createUser(c *gin.Context) {
    var in CreateUser
    if err := c.ShouldBindJSON(&in); err != nil {
        c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
        return
    }
    c.JSON(http.StatusCreated, gin.H{"name": in.Name})
}
```

要点：
- `gin.Default()` = `gin.New()` + Logger + Recovery；生产可自定义中间件。
- 参数：`c.Param`（路径）、`c.Query`（查询）、`c.ShouldBindJSON`（body 绑定 + 校验 `binding:"required"`）。
- 统一响应/错误：自封装 `response.OK(c, data)`、`response.Fail(c, code, msg)`；用中间件统一注册。
- 中间件签名：`func(c *gin.Context)`，调用 `c.Next()` 继续、`c.Abort()` 中断；可用 `c.Set/c.Get` 传值。

---

## 四、Echo 简述

```go
e := echo.New()
e.GET("/users/:id", func(c echo.Context) error {
    return c.JSON(http.StatusOK, map[string]string{"id": c.Param("id")})
})
e.Logger.Fatal(e.Start(":8080"))
```

Echo 路由/参数风格与 Gin 类似（`c.Param`/`c.Bind`/`c.JSON`），返回 `error` 便于集中错误处理中间件。

---

## 五、GORM 数据层

```go
package main

import (
    "gorm.io/driver/sqlite"
    "gorm.io/gorm"
)

type User struct {
    ID   uint   `gorm:"primaryKey"`
    Name string `gorm:"size:64;index"`
    Age  int
}

func main() {
    db, err := gorm.Open(sqlite.Open("app.db"), &gorm.Config{})
    if err != nil { panic(err) }

    db.AutoMigrate(&User{})              // 自动迁移（开发期方便，生产建议用迁移工具）

    db.Create(&User{Name: "alice", Age: 20})
    var u User
    db.First(&u, "name = ?", "alice")    // 查询
    db.Model(&u).Update("age", 21)        // 更新
    db.Delete(&u)                         // 删除

    // 事务
    db.Transaction(func(tx *gorm.DB) error {
        if err := tx.Create(&User{Name: "bob"}).Error; err != nil {
            return err // 返回错误自动回滚
        }
        return nil
    })

    // 预加载关联
    // db.Preload("Orders").Find(&users)

    // 连接池（拿到底层 *sql.DB）
    sqlDB, _ := db.DB()
    sqlDB.SetMaxOpenConns(50)
    sqlDB.SetMaxIdleConns(10)
}
```

要点：
- 用 `?` 占位符防 SQL 注入；避免字符串拼接。
- `Preload` 解决 N+1；批量操作用 `CreateInBatches`。
- 生产谨慎用 `AutoMigrate`，改用版本化迁移（goose / golang-migrate）。
- 软删除：模型含 `gorm.DeletedAt` 字段即可。

---

## 六、go-zero 微服务

go-zero 用 `goctl` 从 `.api` / `.proto` 生成工程骨架：

```text
# user.api
syntax = "v1"
type (
    UserReq  { Id int64 `path:"id"` }
    UserResp { Id int64 `json:"id"`; Name string `json:"name"` }
)
service user-api {
    @handler GetUser
    get /users/:id (UserReq) returns (UserResp)
}
```

```bash
goctl api go -api user.api -dir .   # 生成 handler / logic / routes
```

内置能力：路由与中间件、限流（`PeriodLimit`）、熔断（`Breaker`）、自适应降载、链路追踪、`goctl` 一键生成 CRUD、RPC（zrpc）与服务发现。

适合中大型微服务：api 网关 + 多个 rpc 服务 + 统一治理。

---

## 七、配置、日志与依赖注入

**配置（viper）**
```go
viper.SetConfigName("config")      // config.yaml/json/toml
viper.AddConfigPath("./configs")
viper.AutomaticEnv()               // 支持环境变量覆盖
_ = viper.ReadInConfig()
port := viper.GetInt("server.port")
```

**日志（标准库 log/slog，1.21+）**
```go
logger := slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{Level: slog.LevelInfo}))
slog.SetDefault(logger)
slog.Info("user login", "uid", 42, "ip", "1.2.3.4") // 结构化字段
```

**依赖注入（wire，编译期生成）**
```go
// wire.go
//go:build wireinject
func InitApp() (*App, error) { wire.Build(NewDB, NewRepo, NewService, NewApp); return nil, nil }
```
```bash
wire ./...   # 生成 wire_gen.go（无运行时反射开销）
```

---

## 八、测试与性能分析

**表驱动测试 + 子测试**
```go
func TestAdd(t *testing.T) {
    cases := []struct{ a, b, want int }{{1, 2, 3}, {0, 0, 0}, {-1, 1, 0}}
    for _, c := range cases {
        t.Run(fmt.Sprintf("%d+%d", c.a, c.b), func(t *testing.T) {
            if got := Add(c.a, c.b); got != c.want {
                t.Fatalf("Add(%d,%d)=%d want %d", c.a, c.b, got, c.want)
            }
        })
    }
}
```

**基准测试**：`func BenchmarkAdd(b *testing.B) { for i := 0; i < b.N; i++ { Add(1, 2) } }`，`go test -bench=. -benchmem`。

**HTTP 测试**：`httptest.NewRecorder()` + `http.NewRequest`；Gin 可用 `httptest` 直接打路由。

**常用命令**
```bash
go test ./... -race -cover        # 竞态检测 + 覆盖率
go test -bench=. -benchmem -cpuprofile cpu.out -memprofile mem.out
go tool pprof -http=:9090 cpu.out # 可视化分析
```

**pprof 集成**：引入 `import _ "net/http/pprof"` 并起一个 `http.ListenAndServe("localhost:6060", nil)`，即可 `go tool pprof http://localhost:6060/debug/pprof/heap`。

---

## 九、并发实战模式

**errgroup：并发 + 收集错误 + 统一取消**
```go
import "golang.org/x/sync/errgroup"

g, ctx := errgroup.WithContext(ctx)
for _, url := range urls {
    url := url
    g.Go(func() error { return fetch(ctx, url) })
}
if err := g.Wait(); err != nil { /* 任一失败即返回并取消其它 */ }
```

**限流（带缓冲 channel 信号量）**
```go
sem := make(chan struct{}, 10) // 最大并发 10
for _, task := range tasks {
    sem <- struct{}{}
    go func(t Task) { defer func() { <-sem }(); handle(t) }(task)
}
```

**超时控制**：`ctx, cancel := context.WithTimeout(ctx, 2*time.Second)`，把 `ctx` 一路传到下游 I/O。

**协程池**：高并发场景用 `ants`（`github.com/panjf2000/ants/v2`）复用 goroutine、限制总数。

---

## 十、项目结构与部署

推荐目录（社区通用 layout）：

```text
project/
├─ cmd/            可执行入口（cmd/api/main.go）
├─ internal/       私有业务代码（handler/service/repo/model）
├─ pkg/            可复用的公共库
├─ api/            api/proto 定义
├─ configs/        配置文件
├─ migrations/     数据库迁移
├─ go.mod
└─ Makefile
```

- `internal/` 下的包只能被本项目导入（编译期强制封装）。
- handler（接入层）→ service（业务层）→ repo（数据层）分层，接口隔离便于测试。

**编译与部署**
```bash
CGO_ENABLED=0 go build -trimpath -ldflags "-s -w" -o app ./cmd/api   # 静态二进制、裁剪符号
```

```dockerfile
FROM golang:1.27 AS builder
WORKDIR /app
COPY . .
RUN CGO_ENABLED=0 go build -trimpath -ldflags "-s -w" -o /app/server ./cmd/api

FROM gcr.io/distroless/static
COPY --from=builder /app/server /server
EXPOSE 8080
ENTRYPOINT ["/server"]
```

- 交叉编译：`GOOS=linux GOARCH=amd64 go build`。
- 容器镜像用 `scratch`/`distroless/static` 极小化；设置 `GOMEMLIMIT` 配合容器内存限制。

---

_整理自 Go 官方文档、Gin / GORM / go-zero / slog / wire 等官方文档，仅供学习参考。_
