fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

fn main() {
    let sorted_array = vec![1, 3, 5, 7, 9, 11, 13, 15];
    let target = 9;

    match binary_search(&sorted_array, target) {
        Some(index) => println!("The index of {} is {}", target, index),
        None => println!("{} not found in the array", target),
    }
}


