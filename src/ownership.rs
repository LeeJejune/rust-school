//! 소유권 규칙은 컴파일에 실패하는 코드로도 검증한다.
//! `cargo test --doc`은 아래 코드가 컴파일되지 않아야 성공한다.
//!
//! ### 이동한 String은 다시 사용할 수 없다 (E0382).
//! ```compile_fail,E0382
//! let title = String::from("ownership");
//! let moved = title;
//! println!("{title} {moved}");
//! ```
//!
//! ### 같은 값의 가변 참조 두 개를 겹쳐 사용할 수 없다 (E0499).
//! ```compile_fail,E0499
//! let mut title = String::from("Rust");
//! let first = &mut title;
//! let second = &mut title;
//! first.push('!');
//! second.push('?');
//! ```
//!
//! ### 공유 참조를 나중에도 사용한다면 그 사이에 수정할 수 없다 (E0502).
//! ```compile_fail,E0502
//! let mut title = String::from("Rust");
//! let view = &title;
//! title.push('!');
//! println!("{view}");
//! ```
//!
//! ### 지역 String의 참조를 밖으로 돌려줄 수 없다 (E0515).
//! ```compile_fail,E0515
//! fn invalid<'a>() -> &'a str {
//!     let title = String::from("Rust");
//!     &title
//! }
//! ```
//!
//! 컴파일 실패를 고치려면 참조를 쓸 범위를 줄이거나, 소유한 값을 반환하거나,
//! 정말 독립적인 값이 필요할 때 복제한다. 무조건 clone을 추가하지 않는다.

/// 공백으로 나뉜 첫 단어를 빌려준다. 공백뿐이면 None이다.
/// UTF-8 바이트 인덱스로 자르지 않아 한글에도 사용할 수 있다.
///
/// ```
/// use rust_school::ownership::first_word;
/// let title = String::from("소유권 공부");
/// assert_eq!(first_word(&title), Some("소유권"));
/// assert_eq!(first_word(" \t"), None);
/// ```
pub fn first_word(input: &str) -> Option<&str> {
    input.split_whitespace().next()
}
