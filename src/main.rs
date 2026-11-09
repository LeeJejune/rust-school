use rust_school::{Summary, Task, parse_minutes};

fn main() {
    if let Err(error) = run() {
        eprintln!("오류: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let title = args
        .next()
        .ok_or("사용법: cargo run -- \"소유권 공부\" 30")?;
    let raw_minutes = args.next().ok_or("학습 시간(분)을 입력하세요")?;
    if args.next().is_some() {
        return Err("인수는 제목과 학습 시간 두 개만 입력하세요".into());
    }
    let task = Task::new(title, parse_minutes(&raw_minutes)?)?;
    println!("{}", task.summary());
    Ok(())
}
