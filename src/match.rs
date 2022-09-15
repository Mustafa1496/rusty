fn main() {
    let age = 17;
    match age {
        1..=18 => println!("child"),
        19..=64 => println!("grown"),
        65..=i32::MAX => println!("old"),
        _ => println!("error"),
    };

}
