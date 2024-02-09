fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input = "Hello, World!";
    let reversed = reverse_string(input);
    println!("Reversed string: {}", reversed);
}