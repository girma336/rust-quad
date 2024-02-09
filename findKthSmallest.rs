fn find_kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    let numbers = [4, 2, 6, 1, 3, 5];
    let k = 3;
    match find_kth_smallest(&numbers, k) {
        Some(smallest) => println!("The {}-th smallest number is: {}", k, smallest),
        None => println!("Invalid input"),
    }
}