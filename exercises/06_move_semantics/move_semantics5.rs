#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    // pass a reference to the string, not the string itself 
    // this doesnt take ownership of the string, so we can still use it after this function call 
    get_char(&data);

    // this function takes ownership of the string, so we need to pass the string itself, not a reference to it
    string_uppercase(data);
}
