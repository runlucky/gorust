mod coin;
use coin::Coin;

fn main() {

    let value = Some(5);
    println!("value is: {:?}", add(value));
    println!("value is: {:?}", add(None));

}



fn add(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None
    }
}