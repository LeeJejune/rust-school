//! 학습 기록 앱의 작은 도메인 모델. CLI에서 시작해 HTTP 서버에서도 재사용한다.
//!
//! ```
//! use rust_school::{Task, TaskStatus};
//! let mut task = Task::new(String::from("소유권 공부"), 30)?;
//! task.complete();
//! assert_eq!(task.status(), TaskStatus::Done);
//! # Ok::<(), rust_school::StudyError>(())
//! ```

pub mod ownership;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    Done,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StudyError {
    EmptyTitle,
    InvalidMinutes,
}

impl fmt::Display for StudyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "제목은 공백만으로 구성할 수 없습니다"),
            Self::InvalidMinutes => write!(f, "학습 시간은 1~1440 사이의 정수여야 합니다"),
        }
    }
}

impl std::error::Error for StudyError {}

#[derive(Debug)]
pub struct Task {
    title: String,
    minutes: u32,
    status: TaskStatus,
}

impl Task {
    // String을 받아 내부에 보관하므로 소유권을 넘겨받는다.
    // 공백뿐인 제목은 거부하지만 유효한 제목의 앞뒤 공백은 그대로 보관한다.
    pub fn new(title: String, minutes: u32) -> Result<Self, StudyError> {
        if title.trim().is_empty() {
            return Err(StudyError::EmptyTitle);
        }
        if !(1..=1440).contains(&minutes) {
            return Err(StudyError::InvalidMinutes);
        }
        Ok(Self {
            title,
            minutes,
            status: TaskStatus::Todo,
        })
    }

    // 읽기만 하므로 문자열 복제 없이 빌려준다.
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn minutes(&self) -> u32 {
        self.minutes
    }

    pub fn status(&self) -> TaskStatus {
        self.status
    }

    pub fn rename(&mut self, title: String) -> Result<(), StudyError> {
        if title.trim().is_empty() {
            return Err(StudyError::EmptyTitle);
        }
        self.title = title;
        Ok(())
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Done;
    }
}

pub trait Summary {
    fn summary(&self) -> String;
}

impl Summary for Task {
    fn summary(&self) -> String {
        let status = match self.status {
            TaskStatus::Todo => "할 일",
            TaskStatus::Done => "완료",
        };
        format!("[{status}] {} ({}분)", self.title, self.minutes)
    }
}

/// 사용자 입력을 검증된 학습 시간으로 바꾼다.
pub fn parse_minutes(input: &str) -> Result<u32, StudyError> {
    let minutes = input
        .trim()
        .parse::<u32>()
        .map_err(|_| StudyError::InvalidMinutes)?;
    if !(1..=1440).contains(&minutes) {
        return Err(StudyError::InvalidMinutes);
    }
    Ok(minutes)
}
