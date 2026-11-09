//! 아래 함수 여섯 개를 직접 구현한다. 문제 설명은 docs/07-exercises.md 참고.
//! 기본 테스트에서는 건너뛴다. 한 문제씩 --ignored로 실행한다.

use rust_school::{StudyError, Task, TaskStatus};

fn ex01_double(_minutes: u32) -> u32 {
    todo!("ex01: 두 배 반환. 입력 범위는 0..=1440")
}

fn ex02_take_title(_title: String) -> String {
    todo!("ex02: clone 없이 받은 소유권을 반환")
}

fn ex03_first_word(_input: &str) -> Option<&str> {
    todo!("ex03: 새 String을 만들지 않고 첫 단어 대여")
}

fn ex04_complete_all(_tasks: &mut [Task]) {
    todo!("ex04: 슬라이스의 모든 작업 완료 처리")
}

fn ex05_parse_minutes(_input: &str) -> Result<u32, StudyError> {
    todo!("ex05: 1..=1440 범위 검사, 잘못된 값은 Err")
}

fn ex06_pending_titles(_tasks: &[Task]) -> Vec<&str> {
    todo!("ex06: 순서를 유지하며 미완료 제목을 빌려 반환")
}

#[test]
#[ignore = "직접 구현한 뒤 실행"]
fn ex01() {
    assert_eq!(ex01_double(0), 0);
    assert_eq!(ex01_double(30), 60);
    assert_eq!(ex01_double(1440), 2880);
}

#[test]
#[ignore = "직접 구현한 뒤 실행"]
fn ex02() {
    assert_eq!(ex02_take_title(String::from("소유권")), "소유권");
    assert_eq!(ex02_take_title(String::new()), "");
}

#[test]
#[ignore = "직접 구현한 뒤 실행"]
fn ex03() {
    let input = String::from("  한글\t공부");
    assert_eq!(ex03_first_word(&input), Some("한글"));
    assert_eq!(ex03_first_word("🦀 Rust"), Some("🦀"));
    assert_eq!(ex03_first_word(" \n"), None);
    assert_eq!(ex03_first_word(""), None);
    assert_eq!(input, "  한글\t공부");
}

#[test]
#[ignore = "직접 구현한 뒤 실행"]
fn ex04() {
    let mut tasks = vec![
        Task::new(String::from("기초"), 10).unwrap(),
        Task::new(String::from("대여"), 20).unwrap(),
    ];
    ex04_complete_all(&mut tasks);
    assert!(tasks.iter().all(|task| task.status() == TaskStatus::Done));
    assert_eq!(tasks[0].title(), "기초");
    ex04_complete_all(&mut []);
}

#[test]
#[ignore = "직접 구현한 뒤 실행"]
fn ex05() {
    for valid in ["1", " 30 ", "1440"] {
        assert!(ex05_parse_minutes(valid).is_ok());
    }
    assert_eq!(ex05_parse_minutes(" 30 "), Ok(30));
    for invalid in ["", "0", "-1", "1441", "abc", "4294967296"] {
        assert_eq!(ex05_parse_minutes(invalid), Err(StudyError::InvalidMinutes));
    }
}

#[test]
#[ignore = "직접 구현한 뒤 실행"]
fn ex06() {
    let mut tasks = vec![
        Task::new(String::from("소유권"), 30).unwrap(),
        Task::new(String::from("대여"), 20).unwrap(),
        Task::new(String::from("trait"), 20).unwrap(),
    ];
    tasks[0].complete();
    assert_eq!(ex06_pending_titles(&tasks), ["대여", "trait"]);
    assert_eq!(tasks.len(), 3);
    assert!(ex06_pending_titles(&[]).is_empty());
    for task in &mut tasks {
        task.complete();
    }
    assert!(ex06_pending_titles(&tasks).is_empty());
}
