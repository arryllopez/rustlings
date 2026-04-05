// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    let color = "blue"; 
    let favorite_color = color.to_string();
    return favorite_color;
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
