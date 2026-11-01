# 03_Rust（Rust 方向）

Rust 学习资料整理。内容参照 Rust 官方书《The Rust Programming Language》（`doc.rust-lang.org/book`）、中文文档（`rustwiki.org/zh-CN/book`）与 Rust Reference / Rust by Example 整理。

## 文档

- [Rust 知识笔记合集（整理版）](<Rust知识笔记合集（整理版）.md>) — 17 章：工具链、基础语法、所有权、借用与生命周期、结构体/枚举/模式匹配、集合、错误处理、泛型与 Trait、闭包与迭代器、智能指针、并发、异步、模块与 Cargo、Unsafe 与宏、与 Java/Go 对比、高频面试题。
- [Rust 面试题（精编版）](<Rust面试题（精编版）.md>) — 55 道高频问答 + 7 道「代码能否编译/输出」题，覆盖所有权、Trait、智能指针、并发、异步、工程化与底层。
- [Rust 常用框架与实战指南](<Rust常用框架与实战指南.md>) — Tokio / Axum 0.8 / Actix-web 4 / sqlx / serde / tracing / tower-http、分层结构、部署与容器化。
- 以上三份文档均附同名 `.docx` Word 版本。

## 示例项目

- [axum-todo-api](<示例项目/axum-todo-api>) — Axum 0.8 + Tokio 的最小 Todo REST API（内存存储，可直接 `cargo run`），演示路由/提取器/状态/统一错误/中间件/优雅停机。

## 说明

- 面向有 Java / Go 基础的后端开发者，按「语法 → 所有权 → 类型系统 → 并发 → 工程化」组织。
- 内容参照 Rust 官方书（`doc.rust-lang.org/book`）、中文文档（`rustwiki.org/zh-CN/book`）及 Tokio/Axum/Actix-web/sqlx 等官方文档整理。
- 后续可继续补充：Rust 书籍 PDF、更多实战项目（含数据库/鉴权）。
