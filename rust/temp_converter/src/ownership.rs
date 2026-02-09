

fn ownership1() {
    let s1 = String::from("hello");
    let len = get_length1(s1);
    // s1はムーブされたのでもうアクセスできない
    // println!("{s1}の長さは{len}です");
}

fn get_length1(s: String) -> usize {
    s.len()
}


fn ownership2() {
    let s1 = String::from("hello");
    // Stringを返してもらうことで所有権を取り戻せるが、冗長
    let (s2, len) = get_length2(s1);
    println!("{s2}の長さは{len}です");
}

fn get_length2(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}


pub fn ownership3() {
    let mut s1 = String::from("hello");
    let len = get_length3(&s1);
    // s1は参照渡しされたのでアクセスできる
    println!("{s1}の長さは{len}です");
}
fn get_length3(s: &String) -> usize {
    s.len()
}

pub fn ownership4() {
    let mut s1 = String::from("hello");
    update(&mut s1);
    // s1は可変参照渡しされたので編集もできる
    println!("{s1}");
}
fn update(s: &mut String) {
    s.push_str("world");
}