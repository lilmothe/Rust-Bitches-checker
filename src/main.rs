use std::io;

fn main() {
    println!("Please input your Discord username");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)
        .expect("Failed to read line");
    if username.trim() != "insert your own discord username here lelw" {
        println!("you get no bitches {}", username);
    } else {
        println!("You get bitches");
    }
}
