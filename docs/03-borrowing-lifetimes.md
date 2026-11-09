# 03. 대여, 슬라이스, 수명

근거: [참조와 대여](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html),
[슬라이스](https://doc.rust-lang.org/book/ch04-03-slices.html),
[수명](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).

## 함수 인수가 API 계약이다

| 타입 | 전달하는 권한 | 사용 예 |
| --- | --- | --- |
| `String` | 문자열 소유권 | 저장, 소비, 다른 곳으로 이동 |
| `&str` | 문자열 읽기 대여 | 검증, 검색, 출력 |
| `&mut String` | String 독점 대여와 수정 | push_str로 내용 변경 |
| `&[Task]` | 작업 목록 읽기 | 미완료 작업 검색 |
| `&mut [Task]` | 기존 목록 원소 수정 | 모든 작업 완료 처리 |

참조는 값을 소유하지 않습니다. 대여한 원본보다 오래 사용할 수 없습니다.
일반적인 참조에서는 같은 값에 대한 공유 대여 여러 개 또는 독점적인 가변 대여 하나가 가능합니다.
서로 겹쳐 쓰는 공유·가변 대여는 허용되지 않습니다. 내부 가변성 같은 고급 주제는 이후에 다룹니다.

대여가 필요한 범위는 단순히 중괄호 끝으로만 결정되지 않습니다. 아래는 view의 마지막 사용 후
가변 대여가 시작되므로 동작합니다.

```rust
let mut title = String::from("Rust");
let view = &title;
println!("{view}");
title.push_str(" 공부");
```

마지막 줄 뒤에서 view를 다시 출력하면, 공유 참조를 계속 사용하는 중에 원본을 수정하려 하므로 거부됩니다.

## String과 &str

String은 소유하고 크기를 바꿀 수 있는 문자열입니다. &str은 유효한 UTF-8 문자열 구간을 빌립니다.
읽기 함수는 `&String`보다 `&str`로 받으면 String과 문자열 리터럴 둘 다 편하게 사용할 수 있습니다.
`first_word(&title)`은 새 String을 만들지 않고 입력의 일부를 반환합니다.

문자열의 `len()`은 바이트 수입니다. 한글과 이모지는 여러 바이트일 수 있습니다.
`&text[0..1]`처럼 임의의 바이트로 자르면 UTF-8 문자 경계가 아니어서 panic이 발생할 수 있습니다.
예제에서는 `split_whitespace()`를 사용합니다. `chars().count()`는 Unicode scalar value 개수이며,
화면상 글자(grapheme) 개수와 항상 같지는 않습니다.

## 수명 표기는 생존 시간을 늘려주지 않는다

```rust
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

여기서는 바이트 길이를 비교합니다. 반환 참조를 사용할 수 있는 범위가 두 입력 참조의 유효 범위와
연결된다는 것을 `'a`로 표현합니다. 새로운 메모리를 만들거나 지역 String을 오래 살게 하지 않습니다.
인수가 하나인 `first_word(input: &str) -> Option<&str>`은 이런 관계를 생략 규칙으로 표현할 수 있습니다.

`'static`을 붙이거나 clone을 남발하는 것으로 수명 오류를 덮지 마세요.
소유한 String을 반환해야 하는지, 호출자가 소유한 값을 빌려야 하는지 먼저 결정합니다.

```sh
cargo run --example 03_borrowing
cargo test --doc
rustc --explain E0499
rustc --explain E0502
```

ex03을 풀고, 빈 입력·한글·공백만 있는 입력을 처리하세요.
완료 기준: `String`, `&str`, `&mut String` 중 하나를 이유와 함께 선택할 수 있습니다.
