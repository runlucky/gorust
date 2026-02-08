fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("value: {result}");

}

fn get_value() -> i32 {
    let x = 10;

    return x;
}