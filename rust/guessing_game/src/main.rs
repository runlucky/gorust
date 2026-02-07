use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字あてゲーム！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("秘密の数字は: {secret_number}");

    loop {
        println!("君の予想は？");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込めませんでした");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("数字を入力してください");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい"),
            Ordering::Greater => println!("大きい"),
            Ordering::Equal => {
                println!("あたり！");
                break;
            }
        }

    }


}
