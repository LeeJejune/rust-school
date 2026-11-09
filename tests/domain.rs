use rust_school::{StudyError, Summary, Task, TaskStatus, ownership::first_word, parse_minutes};

#[test]
fn creates_and_completes_a_task_without_losing_its_title() {
    let mut task = Task::new(String::from("소유권 공부"), 30).unwrap();
    assert_eq!(task.status(), TaskStatus::Todo);
    assert_eq!(task.title(), "소유권 공부");
    assert_eq!(task.minutes(), 30);
    task.complete();
    task.complete(); // 완료 처리는 반복해도 완료 상태다.
    assert_eq!(task.status(), TaskStatus::Done);
    assert_eq!(task.summary(), "[완료] 소유권 공부 (30분)");
}

#[test]
fn rejects_blank_titles_and_keeps_previous_title_after_failed_rename() {
    assert!(matches!(
        Task::new(String::from(" \t\n"), 30),
        Err(StudyError::EmptyTitle)
    ));
    let mut task = Task::new(String::from("대여"), 30).unwrap();
    assert_eq!(task.rename(String::new()), Err(StudyError::EmptyTitle));
    assert_eq!(task.title(), "대여");
    task.rename(String::from("가변 대여")).unwrap();
    assert_eq!(task.title(), "가변 대여");
}

#[test]
fn checks_duration_boundaries_in_both_input_and_constructor() {
    for valid in [1, 30, 1440] {
        assert_eq!(parse_minutes(&valid.to_string()), Ok(valid));
        assert!(Task::new(String::from("Rust"), valid).is_ok());
    }
    for invalid in ["", "0", "-1", "1441", "abc", "4294967296"] {
        assert_eq!(parse_minutes(invalid), Err(StudyError::InvalidMinutes));
    }
    assert_eq!(parse_minutes(" 30 "), Ok(30));
    for invalid in [0, 1441, u32::MAX] {
        assert!(matches!(
            Task::new(String::from("Rust"), invalid),
            Err(StudyError::InvalidMinutes)
        ));
    }
}

#[test]
fn finds_unicode_words_and_handles_empty_input() {
    assert_eq!(first_word("  소유권\t공부"), Some("소유권"));
    assert_eq!(first_word("🦀 Rust"), Some("🦀"));
    assert_eq!(first_word(""), None);
    assert_eq!(first_word(" \n\t"), None);
}
