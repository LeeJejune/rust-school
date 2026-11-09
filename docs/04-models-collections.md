# 04. 타입으로 학습 기록 모델 만들기

근거: [Book 5장: struct](https://doc.rust-lang.org/book/ch05-00-structs.html),
[6장: enum](https://doc.rust-lang.org/book/ch06-00-enums.html),
[8장: 컬렉션](https://doc.rust-lang.org/book/ch08-00-common-collections.html).

## struct와 impl

struct는 함께 의미를 갖는 데이터를 묶습니다. `Task`는 제목, 시간, 상태를 가집니다.
`impl Task`는 그 데이터를 생성하거나 다루는 함수들을 모읍니다.

| 선언 | 읽는 방법 |
| --- | --- |
| `Task::new(title, minutes)` | self 없이 호출하는 연관 함수 |
| `task.title()` / `&self` | 원본을 빌려 읽는 메서드 |
| `task.complete()` / `&mut self` | 원본을 독점 대여해 수정하는 메서드 |
| `self` 인수 | 해당 인스턴스의 소유권을 넘겨받는 메서드 |

필드를 비공개로 두고 `new`와 `rename`에서 공백뿐인 제목을 거부합니다.
시간은 1~1440분 범위로 제한합니다. 검증 실패 뒤에 기존 상태가 망가지지 않는지도 테스트합니다.
이 규칙은 이 학습 앱이 정한 요구사항이며 Rust 언어 자체의 제약은 아닙니다.

## enum과 match

문자열 `"todo"`, `"done"` 대신 `TaskStatus::Todo`, `TaskStatus::Done`을 사용합니다.
오타로 생기는 임의 상태를 줄이고, `match`에서 가능한 경우를 다루도록 컴파일러 검사를 받습니다.
enum에는 값도 담을 수 있습니다. Option과 Result가 바로 그 예입니다.

`#[derive(Debug)]`는 디버그 출력을, `PartialEq`는 동등 비교를 지원하는 구현을 생성합니다.
작은 상태 enum에는 Copy를 붙였지만, String을 가진 Task에는 Copy를 붙이지 않았습니다.

## 목록을 소유할까 빌릴까

`Vec<Task>`는 작업을 연속된 목록에 소유하고 길이가 바뀔 수 있습니다.
`for task in &tasks`는 각 작업을 읽고 원래 Vec를 유지합니다.
`for task in &mut tasks`는 각 작업을 수정합니다.
`for task in tasks`는 Vec의 소유권을 받아 Task를 하나씩 이동시킵니다.

`tasks.first()`는 비어 있을 수 있으므로 `Option<&Task>`를 반환합니다.
배열 인덱스로 무조건 접근하기 전에 “없을 때”를 어떻게 처리할지 결정하세요.
주제별 시간을 합산할 때는 `HashMap<String, u32>`와 `entry`를 추가로 연습합니다.

```sh
cargo run --example 04_models
cargo test --test domain
```

ex04는 `&mut [Task]`를 받아 모든 작업을 완료합니다. 빈 슬라이스도 정상 처리해야 합니다.
완료 기준: 상태를 enum으로 표현하고, 목록을 이동시키지 않고 읽기·수정할 수 있습니다.
