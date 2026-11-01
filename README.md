# 编程知识库（Java / Go / Rust）

<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

> 面向后端开发的个人知识库，按 **Java / Go / Rust** 三大方向整理。内容以 **Markdown 文档**为主（每份附同名 Word `.docx`），并包含百度脑图思维导图与示例项目；书籍 PDF 因体积与版权原因不随仓库提供（见 [书籍清单](<00_索引/书籍清单.md>)）。
>
> 整理日期：2026-11-01

## 快速导航

| 方向 | 笔记 | 面试 | 框架 / 实战 |
|---|---|---|---|
| **Java** | [知识笔记合集（整理版）](<01_Java/Java知识笔记合集（整理版）.md>) | [面试题（精编版）](<01_Java/Java面试题（精编版）.md>) | [百度脑图大纲](<01_Java/百度脑图/README.md>) |
| **Go** | [知识笔记合集（整理版）](<02_Go/Go知识笔记合集（整理版）.md>) | [面试题（精编版）](<02_Go/Go面试题（精编版）.md>) | [常用框架与实战指南](<02_Go/Go常用框架与实战指南.md>) |
| **Rust** | [知识笔记合集（整理版）](<03_Rust/Rust知识笔记合集（整理版）.md>) | [面试题（精编版）](<03_Rust/Rust面试题（精编版）.md>) | [常用框架与实战指南](<03_Rust/Rust常用框架与实战指南.md>) · [示例项目](<03_Rust/示例项目/axum-todo-api>) |

## 一、Java（01_Java）

- [Java 知识笔记合集（整理版）](<01_Java/Java知识笔记合集（整理版）.md>) — 14 章：基础与集合、JVM、并发编程、Spring、MySQL、Redis、Netty、计算机网络、Linux、设计模式、数据结构与算法、分布式微服务。
- [Java 面试题（精编版）](<01_Java/Java面试题（精编版）.md>) — 12 章高频面试题：Java 基础、JVM、多线程/线程池/锁、MySQL、Redis、Spring、Spring Boot、RabbitMQ、DDD、Netty、微服务与分布式。
- [百度脑图 · 思维导图](<01_Java/百度脑图/README.md>) — 21 张脑图，合并为 3 册大纲（含 `.docx`）；21 个原始 `.km` 合并为单个 `百度脑图思维导图合集.km`。

## 二、Go（02_Go）

- [Go 知识笔记合集（整理版）](<02_Go/Go知识笔记合集（整理版）.md>) — 17 章：语法、Slice、Map、接口、Context、Channel、GMP 调度、锁与同步、并发控制、内存逃逸、GC、框架生态与排查。
- [Go 面试题（精编版）](<02_Go/Go面试题（精编版）.md>) — 49 道高频问答 + 7 道代码题。
- [Go 常用框架与实战指南](<02_Go/Go常用框架与实战指南.md>) — net/http、Gin、Echo、GORM、go-zero、配置/日志/DI、测试与 pprof、并发模式、部署。

## 三、Rust（03_Rust）

- [Rust 知识笔记合集（整理版）](<03_Rust/Rust知识笔记合集（整理版）.md>) — 17 章：工具链、所有权、借用与生命周期、类型系统、Trait、智能指针、并发、异步、模块与 Cargo、Unsafe 与宏。
- [Rust 面试题（精编版）](<03_Rust/Rust面试题（精编版）.md>) — 55 道高频问答 + 7 道代码题。
- [Rust 常用框架与实战指南](<03_Rust/Rust常用框架与实战指南.md>) — Tokio、Axum 0.8、Actix-web 4、sqlx、serde、tracing、tower-http、分层与部署。
- [axum-todo-api 示例项目](<03_Rust/示例项目/axum-todo-api>) — Axum 0.8 + Tokio 最小 REST API（已通过 `cargo check`）。

## 四、索引与书单（00_索引）

- [知识库索引](<00_索引/知识库索引.md>) — 本仓库文件总览（含 `.docx` 版本）。
- [知识库清单.csv](<00_索引/知识库清单.csv>) — 逐文件全量清单。
- [书籍清单](<00_索引/书籍清单.md>) — 44 本经典书籍书单（PDF 不随仓库提供）。

## 推荐阅读顺序

- **系统复习**：先读对应方向的「知识笔记合集」→ 再看「面试题」检验 → 最后看「框架与实战指南」落地（Rust 可跑示例项目）。
- **面试突击**：直接看「面试题（精编版）」，遇到盲区回到「知识笔记合集」对应章节。
- **脑图速览**：Java 方向可先看 [百度脑图大纲](<01_Java/百度脑图/README.md>) 建立整体框架。

## 目录结构

```text
programming-knowledge-base/
├─ README.md
├─ 00_索引/            知识库索引.md/.docx · 知识库清单.csv · 书籍清单.md
├─ 01_Java/
│  ├─ README.md
│  ├─ Java知识笔记合集（整理版）.md/.docx
│  ├─ Java面试题（精编版）.md/.docx
│  └─ 百度脑图/          合集（一/二/三）.md/.docx + 合集.km + README.md
├─ 02_Go/
│  ├─ README.md
│  ├─ Go知识笔记合集（整理版）.md/.docx
│  ├─ Go面试题（精编版）.md/.docx
│  └─ Go常用框架与实战指南.md/.docx
└─ 03_Rust/
   ├─ README.md
   ├─ Rust知识笔记合集（整理版）.md/.docx
   ├─ Rust面试题（精编版）.md/.docx
   ├─ Rust常用框架与实战指南.md/.docx
   └─ 示例项目/axum-todo-api/   源码（Cargo 项目）
```

## 命名与格式约定

- 每个方向统一三类文档：`<方向>知识笔记合集（整理版）`、`<方向>面试题（精编版）`、`<方向>常用框架与实战指南`（Java 以「百度脑图大纲」替代框架指南）。
- 所有文档同时提供 `.md`（在线阅读）与 `.docx`（下载编辑）两种格式。
- 因 GitHub 单文件预览上限约 100 KB，过大的合集已按主题拆分（如百度脑图合集分为「一 / 二 / 三」册）。
- 已去除重复内容：百度脑图单张 `.md` 大纲全部合并进 3 册合集；21 个原始 `.km` 合并为单个 `百度脑图思维导图合集.km`。

## 免责声明

本仓库内容仅供个人学习与交流使用，书籍版权归原作者及出版社所有。整理内容为个人学习笔记，疏漏在所难免，欢迎指正。

---

_最后整理：2026-11-01_
