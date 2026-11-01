<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# Programming Knowledge Base (Java / Go / Rust)

> A personal, backend-oriented knowledge base organized around three tracks: **Java / Go / Rust**. The content is mainly **Markdown documents** (each with a matching Word `.docx`), plus Baidu mind maps and sample projects. Book PDFs are not included in the repository for size and copyright reasons (see the [Book List](<00_索引/书籍清单.md>)).
>
> Compiled on: 2026-11-01

## Quick Navigation

| Track | Notes | Interview | Framework / Practice |
|---|---|---|---|
| **Java** | [Knowledge Notes (curated)](<01_Java/Java知识笔记合集（整理版）.md>) | [Interview Questions (curated)](<01_Java/Java面试题（精编版）.md>) | [Baidu mind-map outline](<01_Java/百度脑图/README.md>) |
| **Go** | [Knowledge Notes (curated)](<02_Go/Go知识笔记合集（整理版）.md>) | [Interview Questions (curated)](<02_Go/Go面试题（精编版）.md>) | [Common Frameworks & Practice Guide](<02_Go/Go常用框架与实战指南.md>) |
| **Rust** | [Knowledge Notes (curated)](<03_Rust/Rust知识笔记合集（整理版）.md>) | [Interview Questions (curated)](<03_Rust/Rust面试题（精编版）.md>) | [Common Frameworks & Practice Guide](<03_Rust/Rust常用框架与实战指南.md>) · [Sample project](<03_Rust/示例项目/axum-todo-api>) |

## 1. Java (01_Java)

- [Java Knowledge Notes (curated)](<01_Java/Java知识笔记合集（整理版）.md>) — 14 chapters: basics & collections, JVM, concurrency, Spring, MySQL, Redis, Netty, computer networks, Linux, design patterns, data structures & algorithms, distributed microservices.
- [Java Interview Questions (curated)](<01_Java/Java面试题（精编版）.md>) — 12 chapters of high-frequency questions: Java basics, JVM, multithreading/thread pools/locks, MySQL, Redis, Spring, Spring Boot, RabbitMQ, DDD, Netty, microservices & distributed systems.
- [Baidu Mind Maps](<01_Java/百度脑图/README.md>) — 21 mind maps merged into 3 volumes of outlines (with `.docx`); 21 raw `.km` files merged into a single `百度脑图思维导图合集.km`.

## 2. Go (02_Go)

- [Go Knowledge Notes (curated)](<02_Go/Go知识笔记合集（整理版）.md>) — 17 chapters: syntax, Slice, Map, interface, Context, Channel, GMP scheduling, locks & synchronization, concurrency control, memory escape, GC, framework ecosystem & troubleshooting.
- [Go Interview Questions (curated)](<02_Go/Go面试题（精编版）.md>) — 49 high-frequency Q&A + 7 coding problems.
- [Go Common Frameworks & Practice Guide](<02_Go/Go常用框架与实战指南.md>) — net/http, Gin, Echo, GORM, go-zero, config/logging/DI, testing & pprof, concurrency patterns, deployment.

## 3. Rust (03_Rust)

- [Rust Knowledge Notes (curated)](<03_Rust/Rust知识笔记合集（整理版）.md>) — 17 chapters: toolchain, ownership, borrowing & lifetimes, type system, Trait, smart pointers, concurrency, async, modules & Cargo, Unsafe & macros.
- [Rust Interview Questions (curated)](<03_Rust/Rust面试题（精编版）.md>) — 55 high-frequency Q&A + 7 coding problems.
- [Rust Common Frameworks & Practice Guide](<03_Rust/Rust常用框架与实战指南.md>) — Tokio, Axum 0.8, Actix-web 4, sqlx, serde, tracing, tower-http, layering & deployment.
- [axum-todo-api sample project](<03_Rust/示例项目/axum-todo-api>) — a minimal Axum 0.8 + Tokio REST API (passes `cargo check`).

## 4. Index & Book List (00_索引)

- [User Manual](<00_索引/用户操作手册.md>) — how to browse the repo, read the three formats, follow a learning path, import Baidu mind maps, run the sample project, and FAQ.
- [Knowledge Base Index](<00_索引/知识库索引.md>) — full overview of repository files (including `.docx` versions).
- [Knowledge Base Inventory.csv](<00_索引/知识库清单.csv>) — a per-file complete inventory.
- [Book List](<00_索引/书籍清单.md>) — a reading list of 44 classic books (PDFs not included).

## Recommended Reading Order

- **Systematic review**: read the corresponding track's "Knowledge Notes" first → then "Interview Questions" to check yourself → finally the "Frameworks & Practice Guide" to put it into practice (Rust users can run the sample project).
- **Interview sprint**: go straight to "Interview Questions (curated)"; when you hit a gap, return to the matching chapter in "Knowledge Notes".
- **Mind-map overview**: for Java, start with the [Baidu mind-map outline](<01_Java/百度脑图/README.md>) to build the big picture.

## Repository Structure

```text
programming-knowledge-base/
├─ README.md
├─ 00_索引/            knowledgebase index.md/.docx · inventory.csv · book list.md
├─ 01_Java/
│  ├─ README.md
│  ├─ Java知识笔记合集（整理版）.md/.docx
│  ├─ Java面试题（精编版）.md/.docx
│  └─ 百度脑图/          volumes (1/2/3).md/.docx + collection.km + README.md
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
   └─ 示例项目/axum-todo-api/   source (Cargo project)
```

## Naming & Format Conventions

- Each track follows three document types: `<track> Knowledge Notes (curated)`, `<track> Interview Questions (curated)`, `<track> Common Frameworks & Practice Guide` (Java uses a "Baidu mind-map outline" instead of a framework guide).
- Every document is provided in both `.md` (online reading) and `.docx` (download & edit).
- Because GitHub's single-file preview limit is about 100 KB, oversized collections are split by topic (e.g., the Baidu mind-map collection is split into volumes 1/2/3).
- Duplicate content has been removed: all individual Baidu mind-map `.md` outlines were merged into the 3 volumes; the 21 raw `.km` files were merged into a single `百度脑图思维导图合集.km`.

## Disclaimer

This repository is for personal study and exchange only; book copyrights belong to the original authors and publishers. The curated content is personal study notes and inevitably has omissions — corrections are welcome.

---

_Last updated: 2026-11-01_
