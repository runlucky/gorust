mod math;
mod user;
use math::square;
use std::io::stdin;

fn main() {

    let user = get_user();
    user.print();

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