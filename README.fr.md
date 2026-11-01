<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# Base de connaissances en programmation (Java / Go / Rust)

> Une base de connaissances personnelle orientée backend, organisée en trois axes : **Java / Go / Rust**. Le contenu est principalement constitué de **documents Markdown** (chacun accompagné d'un `.docx` Word du même nom), ainsi que de cartes mentales Baidu et de projets d'exemple. Les PDF de livres ne sont pas inclus dans le dépôt pour des raisons de taille et de droits d'auteur (voir la [Liste des livres](<00_索引/书籍清单.md>)).
>
> Date de compilation : 2026-11-01

## Navigation rapide

| Axe | Notes | Entretien | Framework / Pratique |
|---|---|---|---|
| **Java** | [Notes de connaissance (organisées)](<01_Java/Java知识笔记合集（整理版）.md>) | [Questions d'entretien (sélection)](<01_Java/Java面试题（精编版）.md>) | [Plan des cartes mentales Baidu](<01_Java/百度脑图/README.md>) |
| **Go** | [Notes de connaissance (organisées)](<02_Go/Go知识笔记合集（整理版）.md>) | [Questions d'entretien (sélection)](<02_Go/Go面试题（精编版）.md>) | [Frameworks courants et guide pratique](<02_Go/Go常用框架与实战指南.md>) |
| **Rust** | [Notes de connaissance (organisées)](<03_Rust/Rust知识笔记合集（整理版）.md>) | [Questions d'entretien (sélection)](<03_Rust/Rust面试题（精编版）.md>) | [Frameworks courants et guide pratique](<03_Rust/Rust常用框架与实战指南.md>) · [Projet d'exemple](<03_Rust/示例项目/axum-todo-api>) |

## 1. Java (01_Java)

- [Notes Java (organisées)](<01_Java/Java知识笔记合集（整理版）.md>) — 14 chapitres : bases et collections, JVM, concurrence, Spring, MySQL, Redis, Netty, réseaux informatiques, Linux, design patterns, structures de données et algorithmes, microservices distribués.
- [Questions d'entretien Java (sélection)](<01_Java/Java面试题（精编版）.md>) — 12 chapitres de questions fréquentes : bases de Java, JVM, multithreading/pools de threads/verrous, MySQL, Redis, Spring, Spring Boot, RabbitMQ, DDD, Netty, microservices et systèmes distribués.
- [Cartes mentales Baidu](<01_Java/百度脑图/README.md>) — 21 cartes mentales fusionnées en 3 volumes de plans (avec `.docx`) ; les 21 fichiers `.km` originaux fusionnés en un seul `百度脑图思维导图合集.km`.

## 2. Go (02_Go)

- [Notes Go (organisées)](<02_Go/Go知识笔记合集（整理版）.md>) — 17 chapitres : syntaxe, Slice, Map, interfaces, Context, Channel, ordonnancement GMP, verrous et synchronisation, contrôle de concurrence, échappement mémoire, GC, écosystème de frameworks et diagnostic.
- [Questions d'entretien Go (sélection)](<02_Go/Go面试题（精编版）.md>) — 49 questions-réponses fréquentes + 7 problèmes de code.
- [Frameworks courants Go et guide pratique](<02_Go/Go常用框架与实战指南.md>) — net/http, Gin, Echo, GORM, go-zero, configuration/logging/DI, tests et pprof, patterns de concurrence, déploiement.

## 3. Rust (03_Rust)

- [Notes Rust (organisées)](<03_Rust/Rust知识笔记合集（整理版）.md>) — 17 chapitres : chaîne d'outils, ownership, emprunts et lifetimes, système de types, Trait, pointeurs intelligents, concurrence, async, modules et Cargo, Unsafe et macros.
- [Questions d'entretien Rust (sélection)](<03_Rust/Rust面试题（精编版）.md>) — 55 questions-réponses fréquentes + 7 problèmes de code.
- [Frameworks courants Rust et guide pratique](<03_Rust/Rust常用框架与实战指南.md>) — Tokio, Axum 0.8, Actix-web 4, sqlx, serde, tracing, tower-http, architecture en couches et déploiement.
- [Projet d'exemple axum-todo-api](<03_Rust/示例项目/axum-todo-api>) — API REST minimale Axum 0.8 + Tokio (passe `cargo check`).

## 4. Index et liste de livres (00_索引)

- [Index de la base de connaissances](<00_索引/知识库索引.md>) — vue d'ensemble complète des fichiers du dépôt (y compris les versions `.docx`).
- [Inventaire de la base de connaissances.csv](<00_索引/知识库清单.csv>) — inventaire complet fichier par fichier.
- [Liste des livres](<00_索引/书籍清单.md>) — liste de 44 livres classiques (PDF non inclus).

## Ordre de lecture recommandé

- **Révision systématique** : lisez d'abord les « Notes » de l'axe correspondant → puis les « Questions d'entretien » pour vous tester → enfin le « Guide des frameworks » pour la mise en pratique (en Rust, vous pouvez exécuter le projet d'exemple).
- **Préparation d'entretien** : allez directement aux « Questions d'entretien (sélection) » ; en cas de lacune, revenez au chapitre correspondant des « Notes ».
- **Vue d'ensemble par cartes** : pour Java, commencez par le [plan des cartes mentales Baidu](<01_Java/百度脑图/README.md>) pour construire la vue globale.

## Structure du dépôt

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
   └─ 示例项目/axum-todo-api/   code source (projet Cargo)
```

## Conventions de nommage et de format

- Chaque axe suit trois types de documents : `<axe> Notes de connaissance (organisées)`, `<axe> Questions d'entretien (sélection)`, `<axe> Frameworks courants et guide pratique` (Java utilise un « plan des cartes mentales Baidu » à la place du guide de frameworks).
- Chaque document est fourni en `.md` (lecture en ligne) et `.docx` (téléchargement et édition).
- La limite de prévisualisation d'un fichier sur GitHub étant d'environ 100 Ko, les collections trop volumineuses sont découpées par thème (par ex. la collection de cartes mentales Baidu est découpée en volumes 1/2/3).
- Le contenu en double a été supprimé : tous les plans `.md` individuels des cartes mentales Baidu ont été fusionnés dans les 3 volumes ; les 21 fichiers `.km` originaux ont été fusionnés en un seul `百度脑图思维导图合集.km`.

## Avertissement

Le contenu de ce dépôt est destiné uniquement à l'étude et à l'échange personnels ; les droits d'auteur des livres appartiennent à leurs auteurs et éditeurs. Le contenu compilé est constitué de notes personnelles d'étude et comporte inévitablement des omissions — les corrections sont les bienvenues.

---

_Dernière mise à jour : 2026-11-01_
