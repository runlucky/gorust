use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字あてゲーム！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("君の予想は？");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.cmp(&secret_number) {
        Ordering::Less
    }

}
