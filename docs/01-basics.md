# 01. 문법보다 먼저 실행 흐름 잡기

근거: [Book 3장](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html),
[packages와 crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html),
[Cargo Book](https://doc.rust-lang.org/cargo/guide/working-on-an-existing-project.html).

## 코드가 실행되기까지

Rust는 정적 타입 언어입니다. 타입을 생략할 수 있어도 컴파일러가 타입을 추론하고 검사합니다.
컴파일된 프로그램의 시작점은 `fn main()`입니다. `println!`의 `!`는 매크로 호출을 나타냅니다.

이 저장소의 구조를 먼저 봅니다.

| 경로 | 의미 |
| --- | --- |
| Cargo.toml | 패키지 이름, edition, 의존성 선언 |
| Cargo.lock | 실제로 선택된 의존성 버전 기록 |
| src/main.rs | 실행 프로그램인 binary crate의 시작점 |
| src/lib.rs | 여러 실행 프로그램과 테스트가 재사용할 library crate |
| src/ownership.rs | lib.rs에서 `pub mod ownership`으로 연결한 모듈 |
| examples/ | 개별 실행 가능한 학습 예제 |
| tests/ | 공개 API를 바깥에서 사용하는 통합 테스트 |

패키지 이름 `rust-school`은 Rust 코드에서 `rust_school`로 가져옵니다.
crate는 컴파일 단위이고 module은 crate 내부 코드를 묶고 공개 범위를 정하는 단위입니다.

## 꼭 구분할 문법

| 문법 | 의미 | 직접 해볼 것 |
| --- | --- | --- |
| `let x = 10;` | 기본적으로 변경 불가능한 바인딩 | x에 재할당해 오류 읽기 |
| `let mut x = 10;` | 재할당 가능 | x += 1 실행 |
| `let x = x.to_string();` | 같은 이름의 새 바인딩(shadowing) | 숫자에서 문자열로 타입 변경 |
| `u32`, `i32`, `usize` | 부호 없는 정수, 있는 정수, 인덱스에 자주 쓰는 정수 | 음수를 u32에 넣어보기 |
| `bool`, `char` | 참/거짓, Unicode scalar value 하나 | char를 사용자에게 보이는 글자 하나와 무조건 같다고 보지 않기 |
| `[T; N]`, `(A, B)` | 고정 길이 배열, 서로 다른 타입도 담는 튜플 | 배열 반복, 튜플 분해 |
| `fn f(x: u32) -> u32` | 인수와 반환 타입 선언 | 마지막 식의 세미콜론 추가/제거 |

함수의 마지막 `x * 2`는 값이 있는 식입니다. `x * 2;`는 반환값을 만들지 않으므로
반환 타입이 u32인 함수에서 그대로 끝내면 타입이 맞지 않습니다.
`if`도 식이므로 `let level = if ... { ... } else { ... };`처럼 값을 만들 수 있습니다.
반복은 `for item in collection`, 조건 반복은 `while`, 명시적 반복은 `loop`를 사용합니다.

## 실습

```sh
cargo run --example 01_basics
```

출력을 보기 전에 minutes와 total 값을 계산해 보세요. 그다음 `tests/exercises.rs`의 ex01을 풉니다.

완료 기준: 새 함수를 하나 만들고, 인수·반환 타입·변경 가능한 바인딩·마지막 식의 역할을 설명할 수 있습니다.
