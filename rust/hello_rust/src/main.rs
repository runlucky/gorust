fn main() {

    let x = add(5, 10);


    println!("値は{}です", x);
}

fn add(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}