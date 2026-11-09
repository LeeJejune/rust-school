# 06. 공통 동작, 이터레이터, 비동기 진입

근거: [trait](https://doc.rust-lang.org/book/ch10-02-traits.html),
[iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html),
[공유 상태](https://doc.rust-lang.org/book/ch16-03-shared-state.html),
[async/await](https://doc.rust-lang.org/book/ch17-00-async-await.html),
[Tokio 튜토리얼](https://tokio.rs/tokio/tutorial).

## trait와 제네릭

trait는 타입이 제공하는 공통 동작을 정의합니다. `Summary`는 `summary() -> String`을 요구하고,
Task가 그 동작을 구현합니다. `fn print_summary(item: &impl Summary)`는
구체적으로 Task인지 몰라도 Summary 동작을 가진 타입을 빌려 받습니다.
`fn print_summary<T: Summary>(item: &T)`도 같은 의도를 표현하는 기본 형태입니다.

처음부터 모든 코드에 trait를 만들 필요는 없습니다. 구체적인 타입 두 개에 같은 동작이 생기거나,
구현 교체가 실제로 필요해졌을 때 공통 계약을 뽑는 연습을 하세요.

## iterator와 closure

```rust
// tasks는 Vec<Task>라고 가정
// tasks.iter().filter(|task| ...).map(|task| ...).collect::<Vec<_>>()
```

`|task| ...`는 closure입니다. filter로 조건을 고르고 map으로 값을 바꿉니다.
이터레이터 어댑터는 보통 지연 실행되며 collect, sum, for 같은 소비 동작이 있어야 실제 순회합니다.

| 호출 | 기본적인 Vec<T>에서 얻는 원소 | 원본 |
| --- | --- | --- |
| `.iter()` | `&T` | 읽기 대여 |
| `.iter_mut()` | `&mut T` | 수정 대여 |
| `.into_iter()` | `T` | Vec 소유권 소비 |

```sh
cargo run --example 06_traits_iterators
```

ex06은 미완료 제목을 `Vec<&str>`로 반환합니다. 원본 Task를 소비하거나 제목을 clone하지 않습니다.
반환한 참조를 사용하는 동안 원본 목록을 수정하려 하면 어떤 제약이 생길지도 생각해 보세요.

## 서버로 가기 전에 async를 이해하기

`async fn` 호출은 future를 만들며, 그것만으로 작업을 별도 스레드에서 시작하지 않습니다.
future가 poll되어 진행되고, `.await`는 준비되지 않은 작업을 기다리는 지점이 됩니다.
Tokio는 이런 작업을 실행·스케줄링하고 비동기 I/O를 제공하는 런타임입니다.
동시성은 여러 작업을 진행하는 구조이고, 병렬성은 여러 작업을 실제로 동시에 실행하는 것입니다.

서버에서는 다음 세 가지를 순서대로 실습합니다. 코드는 4주차에 의존성을 추가하며 작성합니다.

1. async 함수 하나를 만들고 Tokio 런타임에서 await합니다.
2. 두 개의 I/O 대기를 순차 실행한 경우와 함께 진행한 경우를 비교합니다.
3. 요청 간 상태를 공유하면서 `Arc`와 잠금의 역할을 설명합니다.

`Arc<T>`는 여러 스레드 사이의 공유 소유권을 지원하지만 T의 안전한 변경을 자동으로 제공하지 않습니다.
메모리 실습에서는 `Arc<Mutex<_>>`가 필요할 수 있고, DB 단계에서는 보통 연결 풀을 공유합니다.
잠금을 쥔 채 `.await`하지 않는 설계를 먼저 고민합니다. 긴 CPU 작업이나 동기 블로킹 작업은
런타임 작업을 막을 수 있으므로 별도 처리 방식도 학습합니다.

완료 기준: iter와 into_iter의 소유권 차이를 설명하고, async가 “모든 일을 자동 병렬화한다”는 오해를 피할 수 있습니다.
