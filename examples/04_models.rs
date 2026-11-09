use rust_school::{Summary, Task, TaskStatus};

fn main() -> Result<(), rust_school::StudyError> {
    let mut tasks = vec![
        Task::new(String::from("소유권"), 30)?,
        Task::new(String::from("대여"), 20)?,
    ];
    tasks[0].complete();
    for task in &tasks {
        let next = match task.status() {
            TaskStatus::Todo => "공부하기",
            TaskStatus::Done => "복습하기",
        };
        println!("{} → {next}", task.summary());
    }
    if let Some(first) = tasks.first() {
        println!("첫 학습: {}", first.title());
    }
    Ok(())
}
