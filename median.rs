fn find_median(arr: &[f64]) -> f64 {
    let n = arr.len();
    if n == 0 {
        panic!("Empty array provided.");
    }

    if n % 2 == 0 {
        // For even-length array, median is the average of middle two elements
        let mid_right = n / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] + arr[mid_right]) / 2.0
    } else {
        // For odd-length array, median is the middle element
        arr[n / 2]
    }
}

fn main() {
    let sorted_array1 = [1.0, 2.0, 3.0, 4.0, 5.0];
    let sorted_array2 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    let median1 = find_median(&sorted_array1);
    let median2 = find_median(&sorted_array2);

    println!("Median of array 1: {}", median1);
    println!("Median of array 2: {}", median2);
}
