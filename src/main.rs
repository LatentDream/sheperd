use std::thread;
use std::time::Duration;

fn wait() {
    let ten_millis = Duration::from_millis(10);
    thread::sleep(ten_millis);
}

fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 10;
    let z = x + y;
    println!("The value of z is: {}", z);
    wait();
    let s = String::from("hello");
    println!("{}", s);
}
