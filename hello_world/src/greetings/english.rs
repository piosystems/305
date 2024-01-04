///greeting function simply returns a String
pub fn default_greeting() -> String {
    let message = String::from("Hi!");
    message
}

pub fn default_greeting2() -> String {
    let message: String = String::from(
        "How are you doing?");
    return message;
}