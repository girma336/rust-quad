fn find_median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
        return None;
    }

    let mid = len / 2;

    if len % 2 == 0 {
        Some((arr[mid - 1] + arr[mid]) as f64 / 2.0)
    } else {
        Some(arr[mid] as f64)
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    match find_median(&numbers) {
        Some(median) => println!("The median is: {}", median),
        None => println!("No median found"),
    }
}