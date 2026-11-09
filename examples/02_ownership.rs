fn consume(title: String) -> usize {
    title.len() // 함수 종료 시 이 String이 정리된다. 길이는 바이트 수다.
}

fn main() {
    let title = String::from("Rust");
    let moved = title;
    // println!("{title}"); // 주석을 해제하면 E0382
    let independent = moved.clone(); // 별도 문자열이 필요한 경우의 명시적 복제
    let bytes = consume(moved);
    println!("소비한 문자열: {bytes}바이트 / 별도 값: {independent}");

    let count = 3_u32;
    let copied = count; // u32는 Copy. count도 계속 사용할 수 있다.
    assert_eq!(count + copied, 6);
}
