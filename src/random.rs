
use rand::Rng;

fn main(){
    
    let random_number = rand::thread_rng().gen_range(1..7);
    println!("{}",random_number);
    
}