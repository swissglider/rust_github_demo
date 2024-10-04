/*A Marco Polo Game

If the name Marco is given, the program should respond with Polo.
If the name Marco is not given, the program should respond with What is your name?.

*/

pub fn marco_polo(name: &str) -> String {
    if name.to_lowercase() == "marco" {
        "Polo!".to_string()
    } else {
        "Ich kenne dich nicht!".to_string()
    }
}
