<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# 프로그래밍 지식 베이스 (Java / Go / Rust)

> 백엔드 개발자를 위한 개인 지식 베이스로, **Java / Go / Rust** 세 방향으로 정리했습니다. 내용은 주로 **Markdown 문서**이며(각 문서에 동일 이름의 Word `.docx` 포함), Baidu 마인드맵과 예제 프로젝트도 포함합니다. 도서 PDF는 용량 및 저작권 문제로 저장소에 포함하지 않습니다([도서 목록](<00_索引/书籍清单.md>) 참조).
>
> 정리 날짜: 2026-11-01

## 빠른 탐색

| 방향 | 노트 | 면접 | 프레임워크 / 실전 |
|---|---|---|---|
| **Java** | [지식 노트 모음(정리판)](<01_Java/Java知识笔记合集（整理版）.md>) | [면접 질문(정선판)](<01_Java/Java面试题（精编版）.md>) | [Baidu 마인드맵 개요](<01_Java/百度脑图/README.md>) |
| **Go** | [지식 노트 모음(정리판)](<02_Go/Go知识笔记合集（整理版）.md>) | [면접 질문(정선판)](<02_Go/Go面试题（精编版）.md>) | [자주 쓰는 프레임워크와 실전 가이드](<02_Go/Go常用框架与实战指南.md>) |
| **Rust** | [지식 노트 모음(정리판)](<03_Rust/Rust知识笔记合集（整理版）.md>) | [면접 질문(정선판)](<03_Rust/Rust面试题（精编版）.md>) | [자주 쓰는 프레임워크와 실전 가이드](<03_Rust/Rust常用框架与实战指南.md>) · [예제 프로젝트](<03_Rust/示例项目/axum-todo-api>) |

## 1. Java (01_Java)

- [Java 지식 노트 모음(정리판)](<01_Java/Java知识笔记合集（整理版）.md>) — 14장: 기초와 컬렉션, JVM, 동시성 프로그래밍, Spring, MySQL, Redis, Netty, 컴퓨터 네트워크, Linux, 디자인 패턴, 자료구조와 알고리즘, 분산 마이크로서비스.
- [Java 면접 질문(정선판)](<01_Java/Java面试题（精编版）.md>) — 12장 빈출 면접 질문: Java 기초, JVM, 멀티스레딩/스레드 풀/락, MySQL, Redis, Spring, Spring Boot, RabbitMQ, DDD, Netty, 마이크로서비스와 분산.
- [Baidu 마인드맵](<01_Java/百度脑图/README.md>) — 21개 마인드맵을 3권 개요(`.docx` 포함)로 병합; 21개 원본 `.km`를 단일 `百度脑图思维导图合集.km`로 병합.

## 2. Go (02_Go)

- [Go 지식 노트 모음(정리판)](<02_Go/Go知识笔记合集（整理版）.md>) — 17장: 문법, Slice, Map, 인터페이스, Context, Channel, GMP 스케줄링, 락과 동기화, 동시성 제어, 메모리 이스케이프, GC, 프레임워크 생태계와 문제 해결.
- [Go 면접 질문(정선판)](<02_Go/Go面试题（精编版）.md>) — 빈출 Q&A 49개 + 코드 문제 7개.
- [Go 자주 쓰는 프레임워크와 실전 가이드](<02_Go/Go常用框架与实战指南.md>) — net/http, Gin, Echo, GORM, go-zero, 설정/로깅/DI, 테스트와 pprof, 동시성 패턴, 배포.

## 3. Rust (03_Rust)

- [Rust 지식 노트 모음(정리판)](<03_Rust/Rust知识笔记合集（整理版）.md>) — 17장: 툴체인, 소유권, 차용과 라이프타임, 타입 시스템, Trait, 스마트 포인터, 동시성, 비동기, 모듈과 Cargo, Unsafe와 매크로.
- [Rust 면접 질문(정선판)](<03_Rust/Rust面试题（精编版）.md>) — 빈출 Q&A 55개 + 코드 문제 7개.
- [Rust 자주 쓰는 프레임워크와 실전 가이드](<03_Rust/Rust常用框架与实战指南.md>) — Tokio, Axum 0.8, Actix-web 4, sqlx, serde, tracing, tower-http, 계층화와 배포.
- [axum-todo-api 예제 프로젝트](<03_Rust/示例项目/axum-todo-api>) — Axum 0.8 + Tokio 최소 REST API (`cargo check` 통과).

## 4. 인덱스와 도서 목록 (00_索引)

- [지식 베이스 인덱스](<00_索引/知识库索引.md>) — 저장소 파일 전체 개요(`.docx` 버전 포함).
- [지식 베이스 목록.csv](<00_索引/知识库清单.csv>) — 파일별 전체 목록.
- [도서 목록](<00_索引/书籍清单.md>) — 44권의 고전 도서 목록(PDF 미포함).

## 추천 학습 순서

- **체계적 복습**: 해당 방향의 "지식 노트 모음"을 먼저 읽고 → "면접 질문"으로 점검 → 마지막으로 "프레임워크와 실전 가이드"로 실습(Rust는 예제 프로젝트 실행 가능).
- **면접 돌파**: "면접 질문(정선판)"을 바로 읽고, 막히는 부분은 "지식 노트 모음"의 해당 장으로 돌아가기.
- **마인드맵 개관**: Java는 먼저 [Baidu 마인드맵 개요](<01_Java/百度脑图/README.md>)로 전체 구조 파악.

## 디렉터리 구조

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
   └─ 示例项目/axum-todo-api/   소스 코드(Cargo 프로젝트)
```

## 명명 및 형식 규칙

- 각 방향은 세 종류 문서로 통일: `<방향> 지식 노트 모음(정리판)`, `<방향> 면접 질문(정선판)`, `<방향> 자주 쓰는 프레임워크와 실전 가이드`(Java는 "Baidu 마인드맵 개요"로 프레임워크 가이드 대체).
- 모든 문서는 `.md`(온라인 열람)와 `.docx`(다운로드·편집) 두 형식을 제공.
- GitHub 단일 파일 미리보기 한도가 약 100 KB이므로, 너무 큰 모음은 주제별로 분할(예: Baidu 마인드맵 모음은 "1 / 2 / 3"권으로 분할).
- 중복 내용 제거: Baidu 마인드맵 각 `.md` 개요는 모두 3권 모음으로 병합; 21개 원본 `.km`는 단일 `百度脑图思维导图合集.km`로 병합.

## 면책 조항

본 저장소의 내용은 개인 학습과 교류 목적으로만 제공됩니다. 도서 저작권은 원저자와 출판사에 있습니다. 정리 내용은 개인 학습 노트이며 누락이 있을 수 있으니 지적을 환영합니다.

---

_최종 정리: 2026-11-01_
