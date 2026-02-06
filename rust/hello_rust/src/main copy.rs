fn main() {

    let x = add(5, 10);


    println!("値は{}です", x);

    show_odd_or_even(x);

    // if文を式として使う
    let result = if 10 < x {
        "10より大きい"
    } else {
        "10以下"
    };
    println!("{}", result);

    match x {
        0      => println!("xは0です"),
        1..=10 => println!("xは1から10の間です"),
        _      => println!("xは11以上です"),
    }
}

fn add(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}

fn is_odd(value: i32) -> bool {
    value % 2 != 0
}

fn show_odd_or_even(value: i32) {
    if is_odd(value) {
        println!("{}は奇数です", value);
    } else {
        println!("{}は偶数です", value);
    }
}

