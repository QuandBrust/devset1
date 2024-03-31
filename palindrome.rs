fn is_palindrome(string: &str) -> bool {
    let mut reversed_string = String::new();
    for c in string.chars().rev() {
        reversed_string.push(c);
    }
    return string == reversed_string;
}

fn main() {
    let string = "racecar";
    let is_palindrome = is_palindrome(string);
    println!("The string {} is {}a palindrome", string, if is_palindrome { "" } else { "not "));
}