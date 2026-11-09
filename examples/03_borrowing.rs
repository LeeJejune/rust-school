use rust_school::ownership::first_word;

fn add_suffix(title: &mut String) {
    title.push_str(" 공부");
}

fn main() {
    let mut title = String::from("소유권");
    let view = &title;
    println!("수정 전: {view}"); // view의 마지막 사용
    add_suffix(&mut title); // 이제 공유 대여와 겹치지 않는다.
    assert_eq!(first_word(&title), Some("소유권"));
    assert_eq!(title.chars().count(), 6);
    println!("수정 후: {title}, 바이트 길이: {}", title.len());
}
