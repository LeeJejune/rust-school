use rust_school::{Summary, Task, TaskStatus};

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    // 반환 참조는 두 입력이 모두 유효한 범위에서 사용할 수 있다.
    if left.chars().count() >= right.chars().count() {
        left
    } else {
        right
    }
}

fn main() -> Result<(), rust_school::StudyError> {
    let mut tasks = vec![
        Task::new(String::from("trait"), 30)?,
        Task::new(String::from("iterator"), 20)?,
    ];
    tasks[0].complete();
    let pending: Vec<&str> = tasks
        .iter()
        .filter(|task| task.status() == TaskStatus::Todo)
        .map(|task| task.title())
        .collect();
    assert_eq!(pending, ["iterator"]);
    println!("긴 제목: {}", longer(tasks[0].title(), tasks[1].title()));
    print_summary(&tasks[0]);
    // into_iter()는 Vec의 소유권을 가져가고 Task를 하나씩 이동시킨다.
    for task in tasks {
        print_summary(&task);
    }
    Ok(())
}
