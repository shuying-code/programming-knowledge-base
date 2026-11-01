<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# Base de Conocimiento de Programación (Java / Go / Rust)

> Una base de conocimiento personal orientada al desarrollo backend, organizada en tres áreas: **Java / Go / Rust**. El contenido son principalmente **documentos Markdown** (cada uno con su `.docx` de Word del mismo nombre), además de mapas mentales de Baidu y proyectos de ejemplo. Los PDF de libros no se incluyen en el repositorio por tamaño y derechos de autor (ver la [Lista de libros](<00_索引/书籍清单.md>)).
>
> Fecha de recopilación: 2026-11-01

## Navegación rápida

| Área | Apuntes | Entrevista | Framework / Práctica |
|---|---|---|---|
| **Java** | [Apuntes de conocimiento (curados)](<01_Java/Java知识笔记合集（整理版）.md>) | [Preguntas de entrevista (selección)](<01_Java/Java面试题（精编版）.md>) | [Esquema de mapas mentales Baidu](<01_Java/百度脑图/README.md>) |
| **Go** | [Apuntes de conocimiento (curados)](<02_Go/Go知识笔记合集（整理版）.md>) | [Preguntas de entrevista (selección)](<02_Go/Go面试题（精编版）.md>) | [Frameworks comunes y guía práctica](<02_Go/Go常用框架与实战指南.md>) |
| **Rust** | [Apuntes de conocimiento (curados)](<03_Rust/Rust知识笔记合集（整理版）.md>) | [Preguntas de entrevista (selección)](<03_Rust/Rust面试题（精编版）.md>) | [Frameworks comunes y guía práctica](<03_Rust/Rust常用框架与实战指南.md>) · [Proyecto de ejemplo](<03_Rust/示例项目/axum-todo-api>) |

## 1. Java (01_Java)

- [Apuntes de Java (curados)](<01_Java/Java知识笔记合集（整理版）.md>) — 14 capítulos: bases y colecciones, JVM, concurrencia, Spring, MySQL, Redis, Netty, redes de computadoras, Linux, patrones de diseño, estructuras de datos y algoritmos, microservicios distribuidos.
- [Preguntas de entrevista de Java (selección)](<01_Java/Java面试题（精编版）.md>) — 12 capítulos de preguntas frecuentes: bases de Java, JVM, multihilo/pools de hilos/bloqueos, MySQL, Redis, Spring, Spring Boot, RabbitMQ, DDD, Netty, microservicios y sistemas distribuidos.
- [Mapas mentales de Baidu](<01_Java/百度脑图/README.md>) — 21 mapas mentales fusionados en 3 volúmenes de esquemas (con `.docx`); 21 archivos `.km` originales fusionados en un único `百度脑图思维导图合集.km`.

## 2. Go (02_Go)

- [Apuntes de Go (curados)](<02_Go/Go知识笔记合集（整理版）.md>) — 17 capítulos: sintaxis, Slice, Map, interfaces, Context, Channel, planificación GMP, bloqueos y sincronización, control de concurrencia, escape de memoria, GC, ecosistema de frameworks y diagnóstico.
- [Preguntas de entrevista de Go (selección)](<02_Go/Go面试题（精编版）.md>) — 49 preguntas y respuestas frecuentes + 7 problemas de código.
- [Frameworks comunes de Go y guía práctica](<02_Go/Go常用框架与实战指南.md>) — net/http, Gin, Echo, GORM, go-zero, configuración/logging/DI, pruebas y pprof, patrones de concurrencia, despliegue.

## 3. Rust (03_Rust)

- [Apuntes de Rust (curados)](<03_Rust/Rust知识笔记合集（整理版）.md>) — 17 capítulos: toolchain, ownership, préstamos y lifetimes, sistema de tipos, Trait, punteros inteligentes, concurrencia, async, módulos y Cargo, Unsafe y macros.
- [Preguntas de entrevista de Rust (selección)](<03_Rust/Rust面试题（精编版）.md>) — 55 preguntas y respuestas frecuentes + 7 problemas de código.
- [Frameworks comunes de Rust y guía práctica](<03_Rust/Rust常用框架与实战指南.md>) — Tokio, Axum 0.8, Actix-web 4, sqlx, serde, tracing, tower-http, estratificación y despliegue.
- [Proyecto de ejemplo axum-todo-api](<03_Rust/示例项目/axum-todo-api>) — API REST mínima con Axum 0.8 + Tokio (pasa `cargo check`).

## 4. Índice y lista de libros (00_索引)

- [Índice de la base de conocimiento](<00_索引/知识库索引.md>) — visión general completa de los archivos del repositorio (incluidas las versiones `.docx`).
- [Inventario de la base de conocimiento.csv](<00_索引/知识库清单.csv>) — inventario completo archivo por archivo.
- [Lista de libros](<00_索引/书籍清单.md>) — lista de 44 libros clásicos (PDF no incluidos).

## Orden de lectura recomendado

- **Repaso sistemático**: lee primero los «Apuntes» del área correspondiente → luego «Preguntas de entrevista» para comprobarte → por último la «Guía de frameworks» para ponerlo en práctica (en Rust puedes ejecutar el proyecto de ejemplo).
- **Preparación de entrevistas**: ve directamente a «Preguntas de entrevista (selección)»; cuando encuentres un vacío, vuelve al capítulo correspondiente de los «Apuntes».
- **Vista general con mapas**: para Java, empieza con el [esquema de mapas mentales Baidu](<01_Java/百度脑图/README.md>) para construir la imagen global.

## Estructura del repositorio

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
   └─ 示例项目/axum-todo-api/   código fuente (proyecto Cargo)
```

## Convenciones de nombres y formato

- Cada área sigue tres tipos de documento: `<área> Apuntes de conocimiento (curados)`, `<área> Preguntas de entrevista (selección)`, `<área> Frameworks comunes y guía práctica` (Java usa un «esquema de mapas mentales Baidu» en lugar de la guía de frameworks).
- Cada documento se ofrece en `.md` (lectura en línea) y `.docx` (descarga y edición).
- Como el límite de previsualización de un archivo en GitHub es de unos 100 KB, las colecciones demasiado grandes se dividen por tema (p. ej., la colección de mapas mentales Baidu se divide en los volúmenes 1/2/3).
- Se ha eliminado el contenido duplicado: todos los esquemas `.md` individuales de los mapas mentales Baidu se fusionaron en los 3 volúmenes; los 21 archivos `.km` originales se fusionaron en un único `百度脑图思维导图合集.km`.

## Aviso legal

El contenido de este repositorio es solo para estudio e intercambio personal; los derechos de autor de los libros pertenecen a sus autores y editoriales. El contenido recopilado son apuntes personales de estudio y, sin duda, contiene omisiones — se agradecen las correcciones.

---

_Última actualización: 2026-11-01_
