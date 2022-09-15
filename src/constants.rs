
use std::io;

fn main(){

    const PI: f64 = 3.14159265;

    let mut radius = String::new();

    io::stdin()
    .read_line(&mut radius)
    .expect("didnt read a line");

    let area = PI * radius * radius;

    println!("{:f64}",area);
;}