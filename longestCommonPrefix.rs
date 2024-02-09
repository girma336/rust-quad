fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first = strs[0].as_bytes();

    for (i, &c) in first.iter().enumerate() {
        for word in strs.iter().skip(1) {
            let bytes = word.as_bytes();
            if i >= bytes.len() || c != bytes[i] {
                return String::from_utf8(bytes[..i].to_vec()).unwrap();
            }
        }
    }

    String::from_utf8(first.to_vec()).unwrap()
}

fn main() {
    let words = ["flower", "flow", "flight"];
    let prefix = longest_common_prefix(&words);
    println!("Longest common prefix: {}", prefix);
}