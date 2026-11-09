# 07. 직접 풀 연습문제

수정 대상은 `tests/exercises.rs`의 ex01~ex06 함수 본문입니다.
각 테스트의 기대값을 먼저 읽고, 시그니처는 유지하면서 구현합니다.
테스트를 지우거나 `#[ignore]`만 없애는 것이 풀이가 아닙니다.

## 실행 방법

```sh
# 첫 문제만 실행
cargo test --test exercises ex01 -- --ignored
# 여섯 문제를 모두 구현한 뒤
cargo test --test exercises -- --ignored
```

기본 테스트와 연습문제는 구분됩니다. 기본 `cargo test`가 성공해도 여섯 문제를 푼 것은 아닙니다.
초기 연습 함수에는 todo!가 있으므로 위 명령이 실패하는 것이 정상입니다.

| 문제 | 구현할 것 | 제약 | 힌트 / 연결 노트 |
| --- | --- | --- | --- |
| ex01 | 학습 시간을 두 배로 반환 | 입력은 0..=1440 | 마지막 식, [기초](01-basics.md) |
| ex02 | 받은 String을 그대로 반환 | clone·to_string 없이 | 반환 시 이동, [소유권](02-ownership.md) |
| ex03 | 첫 단어를 Option<&str>로 반환 | 새 String 금지, 공백·한글 처리 | split_whitespace, [대여](03-borrowing-lifetimes.md) |
| ex04 | 슬라이스의 모든 Task 완료 | 목록 길이와 제목 유지 | 가변 순회, [모델](04-models-collections.md) |
| ex05 | 정수를 파싱하고 1..=1440 검사 | trim, 잘못된 입력은 InvalidMinutes | map_err와 ?, [오류](05-error-handling.md) |
| ex06 | 미완료 제목만 순서대로 반환 | clone 금지, 원본 목록 유지 | filter/map/collect, [이터레이터](06-traits-async.md) |

ex03과 ex05는 학습을 위해 기존 라이브러리 함수를 호출하지 말고 같은 동작을 직접 구현합니다.
타입과 테스트는 동작 일부를 검사하지만, “불필요한 clone 없음” 같은 설계 제약까지 모두 증명하지는 않습니다.
완료 후 diff를 읽으면서 제약도 직접 확인하세요.

## 컴파일 오류 읽기 훈련

`src/ownership.rs`에는 컴파일되면 오히려 실패하는 문서 테스트 네 개가 있습니다.
각 코드를 [Rust Playground](https://play.rust-lang.org/) 또는 임시 Rust 파일로 옮겨 직접 고쳐봅니다.

| 오류 | 설명할 것 | 고치는 방향 |
| --- | --- | --- |
| E0382 | 어느 지점에서 값이 이동했나 | 대여로 바꾸기 또는 필요한 소유권 반환 |
| E0499 | 두 가변 참조의 사용 범위가 어디서 겹치나 | 대여를 순서대로 끝내기 |
| E0502 | 공유 참조가 나중에 다시 쓰이나 | 읽기를 마친 뒤 수정 |
| E0515 | 반환 참조의 원본이 언제 정리되나 | 소유한 String 반환 또는 입력 대여 |

각 문제마다 정답 코드만 남기지 말고 아래 세 줄도 기록하세요.

```text
내 예상:
컴파일러가 지적한 위치와 이유:
소유권/대여 관점에서 바꾼 점:
```

문제를 더 풀고 싶으면 Rust 프로젝트의 [Rustlings](https://github.com/rust-lang/rustlings)를 병행합니다.
정답을 붙여넣기 전에 오류 메시지를 읽고 자기 말로 설명하는 것을 완료 기준으로 삼습니다.
