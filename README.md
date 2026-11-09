# Rust School 🦀

Rust 공식 문서를 읽고, 코드를 실행하고, 직접 고치면서 배우는 개인 학습 저장소입니다.
문법 → 소유권·대여 → 타입·오류 처리 → CLI → 웹 API → DB → 화면·배포로 이어갑니다.

설명은 한국어로 새로 작성했으며, 각 노트에 근거가 되는 공식 문서를 연결했습니다.
Rust 언어는 [The Rust Programming Language](https://doc.rust-lang.org/book/)를,
웹 라이브러리는 각 프로젝트가 관리하는 문서를 기준으로 합니다. 확인일: 2026-11-09.

## 바로 시작하기

Rust 설치는 [개발 환경 안내](docs/00-setup.md)를 따릅니다. 설치가 끝났다면:

```sh
git clone https://github.com/LeeJejune/rust-school.git
cd rust-school
cargo run --example 01_basics
cargo run --example 02_ownership
cargo run --example 03_borrowing
cargo run -- "소유권 공부" 30
cargo test
```

마지막 CLI는 `[할 일] 소유권 공부 (30분)`을 출력합니다.
현재는 입력 검증과 출력만 하는 학습용 CLI이며, 파일 저장·HTTP 서버·웹 화면은 다음 실습에서 구현합니다.
기초 코드는 외부 crate 없이 실행됩니다. Rust 2024 Edition과 stable 도구를 사용합니다.

## 읽고 실행하는 순서

| 순서 | 학습 노트 | 실행 예제 / 직접 풀 문제 |
| --- | --- | --- |
| 0 | [설치와 IDE](docs/00-setup.md) | `cargo check` |
| 1 | [기본 문법과 Cargo](docs/01-basics.md) | `01_basics` / ex01 |
| 2 | [소유권: move, Copy, Clone, Drop](docs/02-ownership.md) | `02_ownership` / ex02 |
| 3 | [대여·슬라이스·수명](docs/03-borrowing-lifetimes.md) | `03_borrowing` / ex03 |
| 4 | [struct·enum·컬렉션](docs/04-models-collections.md) | `04_models` / ex04 |
| 5 | [Option·Result·오류 처리](docs/05-error-handling.md) | `05_errors` / ex05 |
| 6 | [trait·제네릭·이터레이터·비동기 진입](docs/06-traits-async.md) | `06_traits_iterators` / ex06 |
| 7 | [연습문제와 채점 방법](docs/07-exercises.md) | `tests/exercises.rs` |
| 8 | [8주 풀스택 로드맵](docs/08-fullstack-roadmap.md) | 학습 기록 앱 확장 |
| 참고 | [공식 문서 모음](docs/09-references.md) | 원문으로 확인하기 |

예제 실행 형식은 `cargo run --example 04_models`입니다.
한 번에 다 읽기보다 노트 하나 → 예제 하나 → 문제 하나 순서로 진행하세요.

## 연습문제는 직접 구현합니다

`tests/exercises.rs`에 `todo!()` 여섯 개가 있습니다. 기본 `cargo test`에서는 의도적으로 건너뜁니다.
학습자가 구현한 뒤 한 문제씩 실행하세요. **초기 상태에서 아래 명령은 실패하는 것이 정상입니다.**

```sh
cargo test --test exercises ex01 -- --ignored
# 모두 구현한 뒤
cargo test --test exercises -- --ignored
```

자동 검증은 완성된 도메인 코드와 문서 예제를 확인합니다.
`cargo test --doc`은 소유권 위반 코드가 실제로 컴파일에 실패하는지도 확인합니다.

## 풀스택 방향

먼저 **Rust(Axum + Tokio) + PostgreSQL(SQLx) + React/TypeScript**로 HTTP·DB·브라우저의 경계를 익히는 경로를 추천합니다.
화면까지 Rust로 쓰고 싶다면 같은 API를 **Leptos CSR** 화면에서 호출한 뒤, 필요할 때 SSR과 server functions로 확장할 수 있습니다.
두 화면을 동시에 만들 필요는 없습니다. 선택 기준, API 설계와 주차별 산출물은 [로드맵](docs/08-fullstack-roadmap.md)에 있습니다.

## 개발 도구

처음에는 **VS Code + rust-analyzer + CodeLLDB**를 추천합니다.
IntelliJ 계열 사용감이 익숙하면 **RustRover**도 좋습니다.
일반적인 명칭은 IDE이며, IDLE는 주로 Python 개발 도구를 가리킵니다.
설치 링크와 디버깅 방법은 [설치와 IDE](docs/00-setup.md)를 참고하세요.

## 변경 후 확인

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`cargo test`에 문서 테스트도 포함됩니다. `--all-targets`만으로 문서 테스트를 대신하지 않습니다.
도구 버전과 실제 검증 결과는 [검증 기록](docs/10-validation.md)에 정리합니다.
