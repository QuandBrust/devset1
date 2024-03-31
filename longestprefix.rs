fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].to_string();

    for s in &strings[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new(); // No common prefix found
            }
        }
    }

    prefix
}

fn main() {
    let words1 = ["flower", "flow", "flight"];
    let words2 = ["dog", "racecar", "car"];

    let prefix1 = longest_common_prefix(&words1);
    let prefix2 = longest_common_prefix(&words2);

    println!("Longest common prefix for words1: {}", prefix1);
    println!("Longest common prefix for words2: {}", prefix2);
}
