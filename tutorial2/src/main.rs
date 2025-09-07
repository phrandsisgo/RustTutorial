use std::io;

fn reply_greeting(name: &str) {
    println!("Hello, {}! how are you?", name);
} 
fn ask_name() -> String {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_string()
}
fn change_name(name: &mut String) {
    name.push_str(" Smith");
}

fn main() {
    let mut user_name = ask_name();
    reply_greeting(&user_name);
    change_name(&mut user_name);
    reply_greeting(&user_name);
}