fn main() {
    let array = [1, 2, 3, 4, 5];

    print!("Element [");
    for element in array {
        print!("{}, ", element);
    }
    println!("]");

    let a1 = (1..5);
    let a2 = (1..5).rev();

    for element in a1 {
        print!("{} ", element);
    }
    println!();
    for element in a2 {
        print!("{} ", element);
    }
    println!();
    

}

fn get_value() -> i32 {
    let x = 10;

    return x;
}