# 10. 검증 기록

검증일: 2026-11-09. 기초 패키지는 외부 crate에 의존하지 않습니다.
현재 검증 대상은 학습용 CLI, 여섯 예제, 도메인 코드, 문서 테스트입니다.
풀스택 로드맵의 서버·DB·화면은 앞으로 구현할 항목입니다.

## 환경

- macOS / Apple Silicon (aarch64-apple-darwin)
- rustc 1.99.1 / cargo 1.99.1
- Rust 2024 Edition
- 검증 도구는 임시 경로에 설치했으며 사용자 셸 설정은 변경하지 않았습니다.

## 확인한 결과

| 확인 | 결과 |
| --- | --- |
| `cargo fmt --all -- --check` | 통과 |
| `cargo clippy --all-targets -- -D warnings` | 경고 없이 통과 |
| `cargo test`의 도메인 테스트 | 4개 통과 |
| 실행 가능한 문서 테스트 | 2개 통과 |
| 소유권 위반 compile_fail 문서 테스트 | 4개 통과: 컴파일 거부 확인 |
| `examples/`의 예제 실행 | 6개 모두 종료 코드 0 |
| CLI 정상·잘못된 입력 시나리오 | 6개 기대한 출력·종료 코드 확인 |
| Markdown 로컬 링크 | 깨진 링크 없음 |

연습문제 여섯 개는 `todo!()`가 있는 학습자용 시작 상태로 남겼고 기본 테스트에서 제외했습니다.
`cargo test --test exercises -- --ignored`는 직접 구현을 마치기 전에는 실패합니다.
통과한 테스트 수에 이 여섯 문제를 포함하지 않았습니다.

## 재검증

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --example 01_basics
cargo run --example 02_ownership
cargo run --example 03_borrowing
cargo run --example 04_models
cargo run --example 05_errors
cargo run --example 06_traits_iterators
cargo run -- "소유권 공부" 30
```

CLI 오류 입력으로 `cargo run -- "Rust" abc`, `cargo run -- "Rust" 0`,
빈 제목, 인수 없음, 인수 초과를 확인했습니다. 오류 입력의 종료 코드 1은 정상 동작입니다.
Rust stable 및 웹 문서는 갱신되므로 향후 의존성 추가 시 해당 버전으로 다시 검증합니다.
