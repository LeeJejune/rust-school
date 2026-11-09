# 09. 원문 찾아보기

문서 확인일: 2026-11-09. 아래는 Rust 프로젝트 또는 해당 도구의 제작자가 관리하는 자료입니다.
이 저장소의 8주 일정과 앱 요구사항은 학습을 위한 제안이며 공식 커리큘럼은 아닙니다.

## 언어와 도구

- [The Rust Programming Language](https://doc.rust-lang.org/book/): 기본 교재. 1~13장을 먼저 학습하고 필요한 고급 장을 추가합니다.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/): 문법과 표준 라이브러리의 실행 예제.
- [Rustlings](https://github.com/rust-lang/rustlings): 오류를 수정하며 푸는 추가 연습.
- [표준 라이브러리](https://doc.rust-lang.org/std/): String, str, Vec, Option, Result 등의 정확한 API.
- [Cargo Book](https://doc.rust-lang.org/cargo/): 패키지·의존성·빌드·workspace 관리.
- [Rust Reference](https://doc.rust-lang.org/reference/): 세부 언어 규칙 확인. 첫 입문 교재보다는 필요할 때 사용합니다.
- [Rust 설치](https://rust-lang.org/tools/install/): rustup과 운영체제별 준비.
- [VS Code의 Rust 안내](https://code.visualstudio.com/docs/languages/rust): 확장·포맷·디버깅.
- [rust-analyzer 설치](https://rust-analyzer.github.io/book/installation.html): 언어 서버와 표준 라이브러리 소스.
- [RustRover](https://www.jetbrains.com/rust/): 통합 IDE와 이용 조건.

## 풀스택

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial): future, task, 공유 상태와 I/O.
- [Axum](https://docs.rs/axum/latest/axum/): 라우팅, extractor, 응답, state.
- [Serde](https://serde.rs/): Rust 데이터와 JSON 등 외부 형식의 직렬화·역직렬화.
- [SQLx](https://docs.rs/sqlx/latest/sqlx/): 비동기 SQL, 연결 풀, 타입 매핑과 migration.
- [PostgreSQL 문서](https://www.postgresql.org/docs/current/): SQL, 제약조건, transaction과 운영.
- [React Learn](https://react.dev/learn): 컴포넌트, state와 화면 갱신.
- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html): 프런트 타입 기초.
- [Vite Guide](https://vite.dev/guide/): 화면 개발 서버와 빌드.
- [Leptos Book](https://book.leptos.dev/): Rust 화면, CSR/SSR, server functions.
- [MDN HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Overview): 요청·응답과 웹 통신.

`latest`나 `current` 문서는 바뀔 수 있습니다. 실제 웹 의존성을 추가할 때는 선택한 버전의 문서를 확인하고
Cargo.lock과 프런트 lockfile을 함께 관리합니다.
