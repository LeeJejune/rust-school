fn double(minutes: u32) -> u32 {
    minutes * 2 // 세미콜론이 없으면 이 식의 값이 반환된다.
}

fn main() {
    let mut minutes = 15;
    minutes += 5;
    let minutes = double(minutes); // shadowing: 새로운 바인딩
    let level = if minutes >= 30 {
        "집중 학습"
    } else {
        "복습"
    };
    let mut total = 0;
    for session in [10, 20, minutes] {
        total += session;
    }
    assert_eq!(total, 70);
    println!("{level}: 총 {total}분");
}
