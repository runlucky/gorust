mod temperature;
mod ownership;

use temperature::Temperature;

fn main() {
    let temp = Temperature::new(40.0, temperature::TemperatureUnit::Celsius);


    println!("celsius: {}", temp.get_value(temperature::TemperatureUnit::Celsius));
    println!("fahrenheit: {}", temp.get_value(temperature::TemperatureUnit::Fahrenheit));

    str();
    str_move();
    str_clone();

    ownership::ownership3();
}

fn str() {
    let mut s1 = String::from("hello, rust");
    let mut s2 = s1;

    // s2 += " appended";

    println!("s2: {}", s2);
    // println!("s1: {}, s2: {}", s1, s2);
}

fn str_move() {
    let s1 = String::from("hello, rust");
    let s2 = s1;

    // s2にムーブされたのでs1を参照するとエラーになる
    // println!("s1: {}", s1);
}

fn str_clone() {
    let s1 = String::from("hello, rust");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);
}

fn ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    // sはtakes_ownershipにムーブされたため、もうアクセスできない
    // println!("{}", s);

    let i = 10;
    makes_copy(i);
    // iはムーブされてもCopyなのでアクセスしてもよい
    println!("{}", i);


}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}