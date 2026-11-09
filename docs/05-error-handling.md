# 05. 실패도 반환 타입에 담기

근거: [Book 6장: Option](https://doc.rust-lang.org/book/ch06-00-enums.html),
[Book 9.2: Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).

## Option과 Result의 차이

| 타입 | 의미 | 이 저장소의 예 |
| --- | --- | --- |
| `Option<T>` | 값이 있거나 없음: Some / None | 첫 단어가 없는 빈 문자열 |
| `Result<T, E>` | 성공 값 또는 실패 이유: Ok / Err | 학습 시간 파싱 실패 |

“값이 없다”가 자연스러운 상태면 Option을, 실패 원인을 호출자가 구분해야 하면 Result를 고려합니다.
예외가 어디서든 튀어나온다고 가정하기보다 함수 시그니처에 적힌 실패를 처리합니다.

## ?의 역할

`let minutes = parse_minutes(input)?;`는 성공이면 내부 u32를 꺼냅니다.
실패이면 현재 함수에서 Err를 반환합니다. `?`가 실패를 무시하거나 자동 재시도하지는 않습니다.
호출 함수가 호환되는 Result를 반환해야 하며, 필요한 경우 오류 타입의 변환도 일어납니다.

`parse_minutes`는 정수 파싱 오류를 `map_err`로 StudyError에 매핑하고,
범위를 검사한 뒤 반환합니다. CLI의 `run()`은 오류를 전달하고,
`main()`에서 사용자 메시지와 종료 코드 1로 바꿉니다.
예제의 `Box<dyn Error>`는 여러 오류를 간단히 전달하기 위한 선택이며,
도메인 함수는 구체적인 StudyError를 반환합니다.

## unwrap과 panic을 쓰는 위치

테스트에서 준비한 유효 데이터는 unwrap으로 간단히 쓸 수 있습니다.
사용자가 입력한 `"abc"`나 외부 파일·DB의 실패를 unwrap으로 처리하면 프로그램이 중단될 수 있습니다.
입력·I/O처럼 예상 가능한 실패는 Result로 다루고, 호출 경계에서 메시지나 HTTP 상태로 바꾸는 연습을 합니다.

```sh
cargo run --example 05_errors
cargo run -- "오류 처리" abc
```

두 번째 명령은 오류 메시지와 종료 코드 1이 정상입니다.
ex05는 정상 값뿐 아니라 0, 음수, 빈 문자열, 범위 초과, 숫자가 아닌 값을 검사합니다.
완료 기준: 실패 경로를 숨기지 않고 Result를 반환하며, `?`가 어느 함수에서 반환하는지 설명할 수 있습니다.
