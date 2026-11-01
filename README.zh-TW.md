<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# 程式設計知識庫（Java / Go / Rust）

> 面向後端開發的個人知識庫，依 **Java / Go / Rust** 三大方向整理。內容以 **Markdown 文件**為主（每份附同名 Word `.docx`），並包含百度腦圖思維導圖與範例專案；書籍 PDF 因體積與版權原因不隨倉庫提供（見 [書籍清單](<00_索引/书籍清单.md>)）。
>
> 整理日期：2026-11-01

## 快速導覽

| 方向 | 筆記 | 面試 | 框架 / 實戰 |
|---|---|---|---|
| **Java** | [知識筆記合集（整理版）](<01_Java/Java知识笔记合集（整理版）.md>) | [面試題（精編版）](<01_Java/Java面试题（精编版）.md>) | [百度腦圖大綱](<01_Java/百度脑图/README.md>) |
| **Go** | [知識筆記合集（整理版）](<02_Go/Go知识笔记合集（整理版）.md>) | [面試題（精編版）](<02_Go/Go面试题（精编版）.md>) | [常用框架與實戰指南](<02_Go/Go常用框架与实战指南.md>) |
| **Rust** | [知識筆記合集（整理版）](<03_Rust/Rust知识笔记合集（整理版）.md>) | [面試題（精編版）](<03_Rust/Rust面试题（精编版）.md>) | [常用框架與實戰指南](<03_Rust/Rust常用框架与实战指南.md>) · [範例專案](<03_Rust/示例项目/axum-todo-api>) |

## 一、Java（01_Java）

- [Java 知識筆記合集（整理版）](<01_Java/Java知识笔记合集（整理版）.md>) — 14 章：基礎與集合、JVM、並發程式設計、Spring、MySQL、Redis、Netty、電腦網路、Linux、設計模式、資料結構與演算法、分散式微服務。
- [Java 面試題（精編版）](<01_Java/Java面试题（精编版）.md>) — 12 章高頻面試題：Java 基礎、JVM、多執行緒/執行緒池/鎖、MySQL、Redis、Spring、Spring Boot、RabbitMQ、DDD、Netty、微服務與分散式。
- [百度腦圖 · 思維導圖](<01_Java/百度脑图/README.md>) — 21 張腦圖，合併為 3 冊大綱（含 `.docx`）；21 個原始 `.km` 合併為單一 `百度脑图思维导图合集.km`。

## 二、Go（02_Go）

- [Go 知識筆記合集（整理版）](<02_Go/Go知识笔记合集（整理版）.md>) — 17 章：語法、Slice、Map、介面、Context、Channel、GMP 排程、鎖與同步、並發控制、記憶體逃逸、GC、框架生態與排查。
- [Go 面試題（精編版）](<02_Go/Go面试题（精编版）.md>) — 49 道高頻問答 + 7 道程式題。
- [Go 常用框架與實戰指南](<02_Go/Go常用框架与实战指南.md>) — net/http、Gin、Echo、GORM、go-zero、設定/日誌/DI、測試與 pprof、並發模式、部署。

## 三、Rust（03_Rust）

- [Rust 知識筆記合集（整理版）](<03_Rust/Rust知识笔记合集（整理版）.md>) — 17 章：工具鏈、所有權、借用與生命週期、型別系統、Trait、智慧指標、並發、非同步、模組與 Cargo、Unsafe 與巨集。
- [Rust 面試題（精編版）](<03_Rust/Rust面试题（精编版）.md>) — 55 道高頻問答 + 7 道程式題。
- [Rust 常用框架與實戰指南](<03_Rust/Rust常用框架与实战指南.md>) — Tokio、Axum 0.8、Actix-web 4、sqlx、serde、tracing、tower-http、分層與部署。
- [axum-todo-api 範例專案](<03_Rust/示例项目/axum-todo-api>) — Axum 0.8 + Tokio 最小 REST API（已通過 `cargo check`）。

## 四、索引與書單（00_索引）

- [知識庫索引](<00_索引/知识库索引.md>) — 本倉庫檔案總覽（含 `.docx` 版本）。
- [知識庫清單.csv](<00_索引/知识库清单.csv>) — 逐檔案全量清單。
- [書籍清單](<00_索引/书籍清单.md>) — 44 本經典書籍書單（PDF 不隨倉庫提供）。

## 建議閱讀順序

- **系統複習**：先讀對應方向的「知識筆記合集」→ 再看「面試題」檢驗 → 最後看「框架與實戰指南」落地（Rust 可跑範例專案）。
- **面試衝刺**：直接看「面試題（精編版）」，遇到盲區回到「知識筆記合集」對應章節。
- **腦圖速覽**：Java 方向可先看 [百度腦圖大綱](<01_Java/百度脑图/README.md>) 建立整體框架。

## 目錄結構

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
   └─ 示例项目/axum-todo-api/   原始碼（Cargo 專案）
```

## 命名與格式慣例

- 每個方向統一三類文件：`<方向>知識筆記合集（整理版）`、`<方向>面試題（精編版）`、`<方向>常用框架與實戰指南`（Java 以「百度腦圖大綱」替代框架指南）。
- 所有文件同時提供 `.md`（線上閱讀）與 `.docx`（下載編輯）兩種格式。
- 因 GitHub 單檔預覽上限約 100 KB，過大的合集已按主題拆分（如百度腦圖合集分為「一 / 二 / 三」冊）。
- 已去除重複內容：百度腦圖單張 `.md` 大綱全部合併進 3 冊合集；21 個原始 `.km` 合併為單一 `百度脑图思维导图合集.km`。

## 免責聲明

本倉庫內容僅供個人學習與交流使用，書籍版權歸原作者及出版社所有。整理內容為個人學習筆記，疏漏在所難免，歡迎指正。

---

_最後整理：2026-11-01_
