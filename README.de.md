<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# Programmier-Wissensdatenbank (Java / Go / Rust)

> Eine persönliche, backend-orientierte Wissensdatenbank, gegliedert in drei Bereiche: **Java / Go / Rust**. Der Inhalt besteht überwiegend aus **Markdown-Dokumenten** (jeweils mit gleichnamigem Word `.docx`), dazu Baidu-Mindmaps und Beispielprojekte. Buch-PDFs werden aus Größen- und Urheberrechtsgründen nicht im Repository bereitgestellt (siehe [Bücherliste](<00_索引/书籍清单.md>)).
>
> Zusammenstellung: 2026-11-01

## Schnellnavigation

| Bereich | Notizen | Interview | Framework / Praxis |
|---|---|---|---|
| **Java** | [Wissensnotizen (überarbeitet)](<01_Java/Java知识笔记合集（整理版）.md>) | [Interviewfragen (Auswahl)](<01_Java/Java面试题（精编版）.md>) | [Baidu-Mindmap-Gliederung](<01_Java/百度脑图/README.md>) |
| **Go** | [Wissensnotizen (überarbeitet)](<02_Go/Go知识笔记合集（整理版）.md>) | [Interviewfragen (Auswahl)](<02_Go/Go面试题（精编版）.md>) | [Gängige Frameworks & Praxisleitfaden](<02_Go/Go常用框架与实战指南.md>) |
| **Rust** | [Wissensnotizen (überarbeitet)](<03_Rust/Rust知识笔记合集（整理版）.md>) | [Interviewfragen (Auswahl)](<03_Rust/Rust面试题（精编版）.md>) | [Gängige Frameworks & Praxisleitfaden](<03_Rust/Rust常用框架与实战指南.md>) · [Beispielprojekt](<03_Rust/示例项目/axum-todo-api>) |

## 1. Java (01_Java)

- [Java-Wissensnotizen (überarbeitet)](<01_Java/Java知识笔记合集（整理版）.md>) — 14 Kapitel: Grundlagen & Collections, JVM, Nebenläufigkeit, Spring, MySQL, Redis, Netty, Computernetze, Linux, Design Patterns, Datenstrukturen & Algorithmen, verteilte Microservices.
- [Java-Interviewfragen (Auswahl)](<01_Java/Java面试题（精编版）.md>) — 12 Kapitel häufiger Fragen: Java-Grundlagen, JVM, Multithreading/Thread-Pools/Locks, MySQL, Redis, Spring, Spring Boot, RabbitMQ, DDD, Netty, Microservices & verteilte Systeme.
- [Baidu-Mindmaps](<01_Java/百度脑图/README.md>) — 21 Mindmaps zu 3 Gliederungsbänden zusammengeführt (mit `.docx`); 21 ursprüngliche `.km`-Dateien zu einer einzigen `百度脑图思维导图合集.km` zusammengeführt.

## 2. Go (02_Go)

- [Go-Wissensnotizen (überarbeitet)](<02_Go/Go知识笔记合集（整理版）.md>) — 17 Kapitel: Syntax, Slice, Map, Interfaces, Context, Channel, GMP-Scheduling, Locks & Synchronisation, Nebenläufigkeitskontrolle, Memory-Escape, GC, Framework-Ökosystem & Fehlersuche.
- [Go-Interviewfragen (Auswahl)](<02_Go/Go面试题（精编版）.md>) — 49 häufige Fragen & Antworten + 7 Codeaufgaben.
- [Gängige Go-Frameworks & Praxisleitfaden](<02_Go/Go常用框架与实战指南.md>) — net/http, Gin, Echo, GORM, go-zero, Konfiguration/Logging/DI, Tests & pprof, Nebenläufigkeitsmuster, Deployment.

## 3. Rust (03_Rust)

- [Rust-Wissensnotizen (überarbeitet)](<03_Rust/Rust知识笔记合集（整理版）.md>) — 17 Kapitel: Toolchain, Ownership, Borrowing & Lifetimes, Typsystem, Trait, Smart Pointer, Nebenläufigkeit, Async, Module & Cargo, Unsafe & Makros.
- [Rust-Interviewfragen (Auswahl)](<03_Rust/Rust面试题（精编版）.md>) — 55 häufige Fragen & Antworten + 7 Codeaufgaben.
- [Gängige Rust-Frameworks & Praxisleitfaden](<03_Rust/Rust常用框架与实战指南.md>) — Tokio, Axum 0.8, Actix-web 4, sqlx, serde, tracing, tower-http, Schichtenarchitektur & Deployment.
- [Beispielprojekt axum-todo-api](<03_Rust/示例项目/axum-todo-api>) — minimale REST-API mit Axum 0.8 + Tokio (besteht `cargo check`).

## 4. Index & Bücherliste (00_索引)

- [Wissensdatenbank-Index](<00_索引/知识库索引.md>) — vollständige Übersicht der Repository-Dateien (inkl. `.docx`-Versionen).
- [Wissensdatenbank-Inventar.csv](<00_索引/知识库清单.csv>) — vollständige dateiweise Inventarliste.
- [Bücherliste](<00_索引/书籍清单.md>) — Liste von 44 klassischen Büchern (PDFs nicht enthalten).

## Empfohlene Lesereihenfolge

- **Systematische Wiederholung**: zuerst die „Wissensnotizen" des jeweiligen Bereichs lesen → dann mit „Interviewfragen" prüfen → zuletzt den „Framework-Leitfaden" in die Praxis umsetzen (bei Rust kann das Beispielprojekt ausgeführt werden).
- **Interview-Vorbereitung**: direkt die „Interviewfragen (Auswahl)" lesen; bei Lücken zum entsprechenden Kapitel der „Wissensnotizen" zurückkehren.
- **Mindmap-Überblick**: für Java zuerst mit der [Baidu-Mindmap-Gliederung](<01_Java/百度脑图/README.md>) das Gesamtbild aufbauen.

## Repository-Struktur

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
   └─ 示例项目/axum-todo-api/   Quellcode (Cargo-Projekt)
```

## Namens- und Formatkonventionen

- Jeder Bereich folgt drei Dokumenttypen: `<Bereich> Wissensnotizen (überarbeitet)`, `<Bereich> Interviewfragen (Auswahl)`, `<Bereich> Gängige Frameworks & Praxisleitfaden` (Java verwendet eine „Baidu-Mindmap-Gliederung" statt des Framework-Leitfadens).
- Jedes Dokument wird in `.md` (Online-Lesen) und `.docx` (Herunterladen & Bearbeiten) bereitgestellt.
- Da die Vorschau einzelner Dateien bei GitHub auf etwa 100 KB begrenzt ist, werden zu große Sammlungen thematisch aufgeteilt (z. B. wird die Baidu-Mindmap-Sammlung in die Bände 1/2/3 aufgeteilt).
- Duplizierter Inhalt wurde entfernt: alle einzelnen `.md`-Gliederungen der Baidu-Mindmaps wurden in die 3 Bände zusammengeführt; die 21 ursprünglichen `.km`-Dateien wurden zu einer einzigen `百度脑图思维导图合集.km` zusammengeführt.

## Haftungsausschluss

Der Inhalt dieses Repositories dient ausschließlich dem persönlichen Lernen und Austausch; die Urheberrechte der Bücher liegen bei den jeweiligen Autoren und Verlagen. Die zusammengestellten Inhalte sind persönliche Lernnotizen und enthalten zwangsläufig Lücken — Korrekturen sind willkommen.

---

_Letzte Aktualisierung: 2026-11-01_
