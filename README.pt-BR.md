<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# Base de Conhecimento de Programação (Java / Go / Rust)

> Uma base de conhecimento pessoal voltada ao desenvolvimento backend, organizada em três eixos: **Java / Go / Rust**. O conteúdo é composto principalmente por **documentos Markdown** (cada um com um `.docx` do Word de mesmo nome), além de mapas mentais do Baidu e projetos de exemplo. Os PDFs de livros não são incluídos no repositório por questões de tamanho e direitos autorais (veja a [Lista de livros](<00_索引/书籍清单.md>)).
>
> Data de organização: 2026-11-01

## Navegação rápida

| Eixo | Anotações | Entrevista | Framework / Prática |
|---|---|---|---|
| **Java** | [Anotações de conhecimento (organizadas)](<01_Java/Java知识笔记合集（整理版）.md>) | [Perguntas de entrevista (seleção)](<01_Java/Java面试题（精编版）.md>) | [Esboço dos mapas mentais Baidu](<01_Java/百度脑图/README.md>) |
| **Go** | [Anotações de conhecimento (organizadas)](<02_Go/Go知识笔记合集（整理版）.md>) | [Perguntas de entrevista (seleção)](<02_Go/Go面试题（精编版）.md>) | [Frameworks comuns e guia prático](<02_Go/Go常用框架与实战指南.md>) |
| **Rust** | [Anotações de conhecimento (organizadas)](<03_Rust/Rust知识笔记合集（整理版）.md>) | [Perguntas de entrevista (seleção)](<03_Rust/Rust面试题（精编版）.md>) | [Frameworks comuns e guia prático](<03_Rust/Rust常用框架与实战指南.md>) · [Projeto de exemplo](<03_Rust/示例项目/axum-todo-api>) |

## 1. Java (01_Java)

- [Anotações de Java (organizadas)](<01_Java/Java知识笔记合集（整理版）.md>) — 14 capítulos: fundamentos e coleções, JVM, concorrência, Spring, MySQL, Redis, Netty, redes de computadores, Linux, padrões de projeto, estruturas de dados e algoritmos, microserviços distribuídos.
- [Perguntas de entrevista de Java (seleção)](<01_Java/Java面试题（精编版）.md>) — 12 capítulos de perguntas frequentes: fundamentos de Java, JVM, multithreading/pools de threads/locks, MySQL, Redis, Spring, Spring Boot, RabbitMQ, DDD, Netty, microserviços e sistemas distribuídos.
- [Mapas mentais Baidu](<01_Java/百度脑图/README.md>) — 21 mapas mentais fundidos em 3 volumes de esboços (com `.docx`); 21 arquivos `.km` originais fundidos em um único `百度脑图思维导图合集.km`.

## 2. Go (02_Go)

- [Anotações de Go (organizadas)](<02_Go/Go知识笔记合集（整理版）.md>) — 17 capítulos: sintaxe, Slice, Map, interfaces, Context, Channel, escalonamento GMP, locks e sincronização, controle de concorrência, escape de memória, GC, ecossistema de frameworks e diagnóstico.
- [Perguntas de entrevista de Go (seleção)](<02_Go/Go面试题（精编版）.md>) — 49 perguntas e respostas frequentes + 7 problemas de código.
- [Frameworks comuns de Go e guia prático](<02_Go/Go常用框架与实战指南.md>) — net/http, Gin, Echo, GORM, go-zero, configuração/logging/DI, testes e pprof, padrões de concorrência, implantação.

## 3. Rust (03_Rust)

- [Anotações de Rust (organizadas)](<03_Rust/Rust知识笔记合集（整理版）.md>) — 17 capítulos: toolchain, ownership, empréstimos e lifetimes, sistema de tipos, Trait, ponteiros inteligentes, concorrência, assíncrono, módulos e Cargo, Unsafe e macros.
- [Perguntas de entrevista de Rust (seleção)](<03_Rust/Rust面试题（精编版）.md>) — 55 perguntas e respostas frequentes + 7 problemas de código.
- [Frameworks comuns de Rust e guia prático](<03_Rust/Rust常用框架与实战指南.md>) — Tokio, Axum 0.8, Actix-web 4, sqlx, serde, tracing, tower-http, estratificação e implantação.
- [Projeto de exemplo axum-todo-api](<03_Rust/示例项目/axum-todo-api>) — API REST mínima com Axum 0.8 + Tokio (passa no `cargo check`).

## 4. Índice e lista de livros (00_索引)

- [Índice da base de conhecimento](<00_索引/知识库索引.md>) — visão geral completa dos arquivos do repositório (incluindo versões `.docx`).
- [Inventário da base de conhecimento.csv](<00_索引/知识库清单.csv>) — inventário completo arquivo por arquivo.
- [Lista de livros](<00_索引/书籍清单.md>) — lista de 44 livros clássicos (PDFs não incluídos).

## Ordem de leitura recomendada

- **Revisão sistemática**: leia primeiro as «Anotações» do eixo correspondente → depois as «Perguntas de entrevista» para se testar → por fim o «Guia de frameworks» para colocar em prática (no Rust você pode executar o projeto de exemplo).
- **Preparação para entrevista**: vá direto às «Perguntas de entrevista (seleção)»; ao encontrar lacunas, volte ao capítulo correspondente das «Anotações».
- **Visão geral por mapas**: para Java, comece pelo [esboço dos mapas mentais Baidu](<01_Java/百度脑图/README.md>) para construir a visão geral.

## Estrutura do repositório

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
   └─ 示例项目/axum-todo-api/   código-fonte (projeto Cargo)
```

## Convenções de nomenclatura e formato

- Cada eixo segue três tipos de documento: `<eixo> Anotações de conhecimento (organizadas)`, `<eixo> Perguntas de entrevista (seleção)`, `<eixo> Frameworks comuns e guia prático` (Java usa um «esboço dos mapas mentais Baidu» em vez do guia de frameworks).
- Cada documento é disponibilizado em `.md` (leitura online) e `.docx` (download e edição).
- Como o limite de pré-visualização de um único arquivo no GitHub é de cerca de 100 KB, coleções muito grandes são divididas por tema (por exemplo, a coleção de mapas mentais Baidu é dividida nos volumes 1/2/3).
- Conteúdo duplicado foi removido: todos os esboços `.md` individuais dos mapas mentais Baidu foram fundidos nos 3 volumes; os 21 arquivos `.km` originais foram fundidos em um único `百度脑图思维导图合集.km`.

## Aviso legal

O conteúdo deste repositório é destinado apenas a estudo e troca pessoais; os direitos autorais dos livros pertencem aos seus autores e editoras. O conteúdo compilado são anotações pessoais de estudo e inevitavelmente contém omissões — correções são bem-vindas.

---

_Última atualização: 2026-11-01_
