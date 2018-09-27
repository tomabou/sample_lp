use std::io;

fn main() {
    println!("Hello, world!");
    println!("Start LP");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("しっぱい");
    println!("{}",guess);
}
