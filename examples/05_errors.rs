use rust_school::{Summary, Task, parse_minutes};

fn from_input(title: String, minutes: &str) -> Result<Task, rust_school::StudyError> {
    let parsed = parse_minutes(minutes)?; // Err면 이 함수에서 즉시 반환
    Task::new(title, parsed)
}

fn main() {
    for input in ["30", "0", "abc", "1441"] {
        match from_input(String::from("오류 처리"), input) {
            Ok(task) => println!("{}", task.summary()),
            Err(error) => println!("{input:?}: {error}"),
        }
    }
}
