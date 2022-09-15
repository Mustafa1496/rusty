
use std::io;

fn main(){

    println!("enter a name");

    let mut name = String::new();

    
    io::stdin()
    .read_line(&mut name)
    .expect("didnt read a line");

    println!("hello {}",name);
}