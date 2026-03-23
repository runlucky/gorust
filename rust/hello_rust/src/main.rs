mod math;
mod user;
use math::square;
use std::io::stdin;

fn main() {
    let user = get_user();
    user.print();
}

// 同時に複数の参照は取得できる
fn mut_test1() {
    let s = String::from("hello");

    let s1 = &s;
    let s2 = &s;
    println!("{}, {}", s1, s2);
}

// 可変参照は他に誰も参照していなければ取得できる
fn mut_test2() {
    let mut s = String::from("hello");

    let s1 = &mut s;
    s1.push_str(", world");
    println!("{}", s1);
}

fn please_input_number() {
    println!("数字を入力してください: ");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // 数字じゃなかったらエラー表示
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("エラー: {}", e);
            return;
        }
    };

    println!("2乗した値は{}です", square(number));
    println!("2倍した値は{}です", math::double(number));
}

fn get_user() -> user::User {
    user::User {
        name: String::from("太郎"),
        age: 30,
    }
}
