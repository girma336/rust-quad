fn is_palindrome(s: &str) -> bool {
    let len = s.len();
    let chars: Vec<char> = s.chars().collect();

    for i in 0..len / 2 {
        if chars[i] != chars[len - i - 1] {
            return false;
        }
    }

    true
}

fn main() {
    println!("{}", is_palindrome("aaaaa"));
}